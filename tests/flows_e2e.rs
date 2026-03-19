//! End-to-end test that creates a payment session, serves a Checkout.com Flow
//! page via an inline Axum server, drives headless Chrome to fill in test card
//! details, and asserts the payment succeeds.

use axum::{Router, extract::State, response::Html, routing::get};
use checkout::Client;
use checkout::models::flows::CreatePaymentSessionRequest;
use checkout::models::shared::{Address, BillingInformation, Currency};
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Shared helpers (same as flows.rs)
// ---------------------------------------------------------------------------

fn client() -> Option<Client> {
    dotenvy::dotenv().ok();
    Client::from_env().ok()
}

fn valid_address() -> Address {
    Address::builder()
        .address_line1("123 Test St")
        .city("London")
        .zip("W1T 4TJ")
        .country("GB")
        .build()
}

fn valid_billing() -> BillingInformation {
    BillingInformation::builder()
        .address(valid_address())
        .build()
}

// ---------------------------------------------------------------------------
// Inline Axum server (replaces flow_frontend crate)
// ---------------------------------------------------------------------------

struct AppState {
    public_key: String,
    payment_session_json: String,
}

/// Start an Axum server on a random port serving the Checkout Flow HTML page.
/// Returns the port the server is listening on.
async fn start_flow_server(public_key: String, payment_session_json: String) -> u16 {
    let state = Arc::new(AppState {
        public_key,
        payment_session_json,
    });

    let app = Router::new().route("/", get(index)).with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    port
}

async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let html = include_str!("flows_e2e.html")
        .replace("__PUBLIC_KEY__", &state.public_key)
        .replace("__PAYMENT_SESSION_JSON__", &state.payment_session_json);

    Html(html)
}

// ---------------------------------------------------------------------------
// E2E test
// ---------------------------------------------------------------------------

/// Test card number for sandbox (Visa success).
const TEST_CARD_NUMBER: &str = "4242424242424242";
/// Test expiry date (MMYY).
const TEST_CARD_EXPIRY: &str = "1028";
/// Test CVV.
const TEST_CARD_CVV: &str = "100";
/// Test cardholder name.
const TEST_CARDHOLDER_NAME: &str = "John Doe";

/// Timeout (seconds) for waiting on the payment result UI.
const PAYMENT_RESULT_TIMEOUT_SECS: u64 = 15;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn payment_session_request_processed_e2e() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    let Ok(public_key) = std::env::var("CKO_PUBLIC_KEY") else {
        println!("Skipping E2E test: CKO_PUBLIC_KEY is missing (required for Flow client)");
        return;
    };

    let request = CreatePaymentSessionRequest::builder()
        .amount(2500)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-e2e")
        .billing(valid_billing())
        .success_url("http://localhost:4444/success") // local dummy url
        .failure_url("http://localhost:4444/failure") // local dummy url
        .build();

    let response = client
        .flows()
        .create_payment_session(&request)
        .await
        .unwrap();

    // --- Start inline Axum server ---

    let session_json = serde_json::to_string(&response).unwrap();
    let port = start_flow_server(public_key, session_json).await;

    let test_url = format!("http://127.0.0.1:{port}");
    println!("Test URL: {}", test_url);

    let browser_opts = headless_chrome::LaunchOptions::default_builder()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .args(vec![
            std::ffi::OsStr::new("--disable-web-security"),
            std::ffi::OsStr::new("--disable-features=IsolateOrigins,site-per-process"),
            std::ffi::OsStr::new("--disable-site-isolation-trials"),
        ])
        .build()
        .unwrap();

    let browser =
        headless_chrome::Browser::new(browser_opts).expect("Failed to launch headless chrome");

    let tab = browser.new_tab().expect("Failed to get new tab");

    println!("Navigating to Test URL: {}", test_url);
    tab.navigate_to(&test_url).expect("failed to navigate");

    tab.wait_for_element("#flow-container")
        .expect("failed to wait for flow-container");

    println!("Waiting for Flow Component to load iframes...");
    // Give time for CheckoutWebComponents to initialize and inject their iframes
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;

    // Fill Cardholder Name
    tab.evaluate("window.focusCardholderName()", false).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    tab.type_str(TEST_CARDHOLDER_NAME).unwrap();

    // Focus first iframe (card number)
    tab.evaluate("window.focusCardNumber()", false).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    tab.type_str(TEST_CARD_NUMBER).unwrap();

    // Focus second iframe (expiry)
    tab.evaluate("window.focusCardExpiry()", false).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    tab.type_str(TEST_CARD_EXPIRY).unwrap();

    // Focus third iframe (cvv)
    tab.evaluate("window.focusCardCvv()", false).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    tab.type_str(TEST_CARD_CVV).unwrap();

    // Allow the component a moment to process the filled fields
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // The DOM has multiple buttons (e.g. the accordion button). Add an ID to the actual Pay button to click natively.
    tab.evaluate("window.preparePayButton()", false).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let submit = tab.wait_for_element("#checkout-pay-button").unwrap();
    submit.click().unwrap();

    // Poll until the payment result appears in the body text
    let start = std::time::Instant::now();
    let mut success = false;

    loop {
        if start.elapsed().as_secs() > PAYMENT_RESULT_TIMEOUT_SECS {
            break;
        }
        let body_text: String = tab
            .evaluate("window.getBodyText()", false)
            .unwrap()
            .value
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();
        if body_text.contains("Payment complete") {
            success = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    println!("Checkout success UI marked 'Payment complete': {}", success);

    // Print captured Javascript console errors
    let logs_json = tab.evaluate("window.getCapturedLogs()", false).unwrap();
    println!("JS LOGS: {:?}", logs_json);

    assert!(success, "Payment did not succeed in UI");
}
