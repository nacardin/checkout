use base64::Engine;
use checkout::models::flows::{CreatePaymentSessionRequest, CreatePaymentSessionResponse};
use checkout::models::shared::{Address, BillingInformation, Currency, CustomerDescriptor};
use checkout::{Client, Error, StatusCode};

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

fn valid_customer() -> CustomerDescriptor {
    CustomerDescriptor::builder()
        .email("test@example.com")
        .name("Test User")
        .build()
}

fn assert_api_error(
    response: Result<CreatePaymentSessionResponse, Error>,
    expected_status: StatusCode,
    expected_error_type: &str,
    expected_error_code: &str,
) {
    let Err(Error::Api { status_code, error }) = response else {
        panic!("Expected Api error");
    };

    assert_eq!(status_code, expected_status);
    assert_eq!(error.error_type, expected_error_type);
    assert!(
        error
            .error_codes
            .iter()
            .any(|code| code == expected_error_code),
        "Expected error code '{}' not found in {:?}",
        expected_error_code,
        error.error_codes
    );
}

#[tokio::test]
async fn payment_session_request_created() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };
    let request = CreatePaymentSessionRequest::builder()
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test")
        .billing(valid_billing())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client
        .flows()
        .create_payment_session(&request)
        .await
        .unwrap();

    println!("Response: {:#?}", response);

    assert!(response.id.starts_with("ps_"));
    assert!(response.payment_session_secret.starts_with("pss_"));
}

#[tokio::test]
async fn payment_session_request_invalid_processing_channel_id() {
    let Some(client) = client() else { return };
    let processing_channel_id = "invalid_channel_id".to_string();

    let request = CreatePaymentSessionRequest::builder()
        .amount(0)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-invalid-processing-channel-id")
        .billing(valid_billing())
        .customer(valid_customer())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client.flows().create_payment_session(&request).await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "processing_channel_id_invalid",
    );
}

#[tokio::test]
async fn payment_session_request_processed_with_invalid_customer_id() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    let request = CreatePaymentSessionRequest::builder()
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-invalid-customer-id")
        .billing(valid_billing())
        .customer(CustomerDescriptor::builder().id("cus_1234567890").build())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client.flows().create_payment_session(&request).await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "customer_id_invalid",
    );
}

#[tokio::test]
async fn payment_session_request_invalid_customer_email() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    let request = CreatePaymentSessionRequest::builder()
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-invalid-customer-email")
        .billing(valid_billing())
        .customer(
            CustomerDescriptor::builder()
                .email("invalid-email")
                .name("Test User")
                .build(),
        )
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client.flows().create_payment_session(&request).await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "customer_email_invalid",
    );
}

#[tokio::test]
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
        .amount(2000)
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

    // --- Spawn flow_frontend instead of inline webserver ---

    // Build the flow_frontend binary
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let frontend_manifest = std::path::PathBuf::from(manifest_dir)
        .join("flow_frontend")
        .join("Cargo.toml");

    let build_status = std::process::Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            frontend_manifest.to_str().unwrap(),
        ])
        .status()
        .expect("failed to run cargo build");
    assert!(build_status.success(), "cargo build flow_frontend failed");

    // Resolve the binary path from the target directory
    let binary = std::process::Command::new("cargo")
        .args([
            "metadata",
            "--format-version=1",
            "--no-deps",
            "--manifest-path",
            frontend_manifest.to_str().unwrap(),
        ])
        .output()
        .expect("failed to run cargo metadata");
    let meta: serde_json::Value = serde_json::from_slice(&binary.stdout).unwrap();
    let target_dir = meta["target_directory"].as_str().unwrap();
    let binary_path = std::path::PathBuf::from(target_dir)
        .join("debug")
        .join("flow_frontend");

    // Spawn with --port 0 so the OS picks a free port; stdout is piped
    // so we can read the LISTENING_PORT=<N> line.
    // Base64-encode the full payment session response so the frontend has all fields
    let session_json = serde_json::to_string(&response).unwrap();
    let session_b64 = base64::engine::general_purpose::STANDARD.encode(&session_json);

    let mut child = tokio::process::Command::new(&binary_path)
        .args(["--payment-session", &session_b64, "--port", "0"])
        .env("CKO_PUBLIC_KEY", &public_key)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .expect("failed to spawn flow_frontend");

    // Read stdout until we see the LISTENING_PORT=<N> line
    use tokio::io::{AsyncBufReadExt, BufReader};
    let stdout = child.stdout.take().expect("stdout not captured");
    let mut reader = BufReader::new(stdout).lines();

    let port: u16 = loop {
        let line = tokio::time::timeout(std::time::Duration::from_secs(10), reader.next_line())
            .await
            .expect("timed out waiting for flow_frontend to print port")
            .expect("failed to read stdout")
            .expect("flow_frontend exited before printing port");

        eprintln!("[flow_frontend] {line}");
        if let Some(port_str) = line.strip_prefix("LISTENING_PORT=") {
            break port_str.parse().expect("invalid port number");
        }
    };

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

    let tab = browser
        .new_tab()
        .expect("Failed to get new tab");

    println!("Navigating to Test URL: {}", test_url);
    tab.navigate_to(&test_url).expect("failed to navigate");

    tab.wait_for_element("#flow-container")
        .expect("failed to wait for flow-container");

    println!("Waiting for Flow Component to load iframes...");
    // Give time for CheckoutWebComponents to initialize and inject their iframes
    std::thread::sleep(std::time::Duration::from_secs(4));

    // Fill Cardholder Name
    tab.evaluate(
        "let el = document.querySelector('input[name=\"cardholderName\"], input[id=\"cardholderName\"], input[autocomplete=\"cc-name\"], iframe[data-testid*=\"cardholder\"]'); if(el) el.focus();",
        false
    ).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    let _ = tab.type_str("John Doe");

    // Focus first iframe (card)
    tab.evaluate("document.querySelector(\"iframe[data-testid='card-number']\").focus()", false).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    tab.type_str("4242424242424242").unwrap();

    // Focus second iframe (expiry)
    tab.evaluate("document.querySelector(\"iframe[data-testid='card-expiry-date']\").focus()", false).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    tab.type_str("1028").unwrap();

    // Focus third iframe (cvv)
    tab.evaluate("document.querySelector(\"iframe[data-testid='card-cvv']\").focus()", false).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    tab.type_str("100").unwrap();

    // Sometimes the component takes a moment
    std::thread::sleep(std::time::Duration::from_millis(500));

    // The DOM has multiple buttons (e.g. the accordion button). Add an ID to the actual Pay button to click natively.
    tab.evaluate(
        r#"
        let b = Array.from(document.querySelectorAll('button')).find(btn => btn.textContent.includes('Pay') || btn.innerText.includes('Pay')); 
        if(b) { 
            b.id = 'checkout-pay-button'; 
            b.scrollIntoView({block: 'center'}); 
        } 
        else { console.error('Pay button not found'); }
        "#, 
        false
    ).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    let submit = tab.wait_for_element("#checkout-pay-button").unwrap();
    submit.click().unwrap();

    // Give time for the simulated submission to process and the UI to update
    let start = std::time::Instant::now();
    let mut success = false;
    
    loop {
        if start.elapsed().as_secs() > 15 {
            break;
        }
        let body_text: String = tab.evaluate("document.body.innerText", false).unwrap().value.unwrap().as_str().unwrap().to_owned();
        if body_text.contains("Payment complete") {
            success = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    println!("Checkout success UI marked 'Payment complete': {}", success);
    
    // Print captured Javascript console errors
    let logs_script = "window._capturedLogs ? JSON.stringify(window._capturedLogs) : '[]'";
    let logs_json = tab.evaluate(logs_script, false).unwrap();
    println!("JS LOGS: {:?}", logs_json);
    
    assert!(success, "Payment did not succeed in UI");
}
