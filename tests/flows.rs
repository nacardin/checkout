use std::fmt::Debug;

use checkout::models::flows::{
    CreateAndSubmitPaymentSessionRequest, CreatePaymentSessionRequest, SubmitPaymentSessionRequest,
};
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

fn assert_api_error<T: Debug>(
    response: Result<T, Error>,
    expected_status: StatusCode,
    expected_error_type: &str,
    expected_error_code: &str,
) {
    let Err(Error::Api { status_code, error }) = response else {
        panic!("Expected Api error, got: {response:?}");
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

// ---------------------------------------------------------------------------
// Create Payment Session tests
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Submit Payment Session tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn submit_payment_session_not_found() {
    let Some(client) = client() else { return };

    let request = SubmitPaymentSessionRequest::builder()
        .session_data("fake_session_data_token")
        .build();

    let response = client
        .flows()
        .submit_payment_session("ps_nonexistent_id", &request)
        .await;

    println!("Response: {:#?}", response);

    if let Err(err) = response {
        assert!(
            matches!(
                err,
                Error::UnexpectedStatusCode(StatusCode::NOT_FOUND) | Error::Api { .. }
            ),
            "Expected 404 or API error, got: {err:?}"
        );
    } else {
        panic!("Expected an error for non-existent payment session ID");
    }
}

#[tokio::test]
async fn submit_payment_session_invalid_session_data() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    // First create a valid payment session
    let create_request = CreatePaymentSessionRequest::builder()
        .amount(3000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-submit-invalid-session-data")
        .billing(valid_billing())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let session = client
        .flows()
        .create_payment_session(&create_request)
        .await
        .unwrap();

    // Then try to submit with an invalid session_data token
    let submit_request = SubmitPaymentSessionRequest::builder()
        .session_data("invalid_session_data_token")
        .build();

    let response = client
        .flows()
        .submit_payment_session(&session.id, &submit_request)
        .await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "session_data_invalid",
    );
}

#[tokio::test]
async fn submit_payment_session_with_amount_override() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    // Create a valid payment session
    let create_request = CreatePaymentSessionRequest::builder()
        .amount(5000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-submit-amount-override")
        .billing(valid_billing())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let session = client
        .flows()
        .create_payment_session(&create_request)
        .await
        .unwrap();

    // Submit with amount override and invalid session_data
    let submit_request = SubmitPaymentSessionRequest::builder()
        .session_data("invalid_session_data")
        .amount(4500_u64)
        .reference("overridden-ref")
        .build();

    let response = client
        .flows()
        .submit_payment_session(&session.id, &submit_request)
        .await;

    println!("Response: {:#?}", response);

    // Will fail because session_data is invalid, but this validates
    // the request serialization with optional fields works correctly
    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "session_data_invalid",
    );
}

// ---------------------------------------------------------------------------
// Create and Submit Payment Session tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn create_and_submit_invalid_processing_channel_id() {
    let Some(client) = client() else { return };

    let request = CreateAndSubmitPaymentSessionRequest::builder()
        .session_data("fake_session_data_token")
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id("invalid_channel_id")
        .reference("rust-sdk-test-cas-invalid-channel")
        .billing(valid_billing())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client
        .flows()
        .create_and_submit_payment_session(&request)
        .await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "processing_channel_id_invalid",
    );
}

#[tokio::test]
async fn create_and_submit_invalid_session_data() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    let request = CreateAndSubmitPaymentSessionRequest::builder()
        .session_data("invalid_session_data_token")
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-cas-invalid-session-data")
        .billing(valid_billing())
        .success_url("https://example.com/success")
        .failure_url("https://example.com/failure")
        .build();

    let response = client
        .flows()
        .create_and_submit_payment_session(&request)
        .await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "session_data_invalid",
    );
}

#[tokio::test]
async fn create_and_submit_invalid_customer_email() {
    let Some(client) = client() else { return };
    let Ok(processing_channel_id) = std::env::var("CKO_PROCESSING_CHANNEL_ID") else {
        return;
    };

    let request = CreateAndSubmitPaymentSessionRequest::builder()
        .session_data("fake_session_data")
        .amount(2000)
        .currency(Currency::USD)
        .processing_channel_id(processing_channel_id)
        .reference("rust-sdk-test-cas-invalid-email")
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

    let response = client
        .flows()
        .create_and_submit_payment_session(&request)
        .await;

    println!("Response: {:#?}", response);

    assert_api_error(
        response,
        StatusCode::UNPROCESSABLE_ENTITY,
        "validation_error",
        "customer_email_invalid",
    );
}
