use crate::{Client, Error};

use crate::models::flows::{
    CreateAndSubmitPaymentSessionRequest, CreateAndSubmitPaymentSessionResponse,
    CreatePaymentSessionRequest, CreatePaymentSessionResponse, SubmitPaymentSessionRequest,
    SubmitPaymentSessionResponse,
};

/// Access the Flows API.
#[derive(Debug, Clone)]
pub struct Flows<'a> {
    client: &'a Client,
}

impl<'a> Flows<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Creates a payment session for the Flow integration.
    ///
    /// The values you provide in the request will be used to determine the
    /// payment methods available to Flow. You must supply the unmodified
    /// response body when you initialize Flow.
    ///
    /// [`POST /payment-sessions`](https://api-reference.checkout.com/#operation/CreatePaymentSession)
    pub async fn create_payment_session(
        &self,
        request: &CreatePaymentSessionRequest,
    ) -> Result<CreatePaymentSessionResponse, Error> {
        let url = format!("{}/payment-sessions", self.client.environment.api_url());
        self.client
            .send_post_request("payment-sessions", &url, request)
            .await
    }

    /// Submit a payment attempt for an existing payment session.
    ///
    /// This request works with the Flow `handleSubmit` callback, where you can
    /// perform a customized payment submission. You must send the unmodified
    /// response body as the response of the `handleSubmit` callback.
    ///
    /// [`PUT /payment-sessions/{id}/submit`](https://api-reference.checkout.com/#operation/SubmitPaymentSession)
    pub async fn submit_payment_session(
        &self,
        payment_session_id: &str,
        request: &SubmitPaymentSessionRequest,
    ) -> Result<SubmitPaymentSessionResponse, Error> {
        let url = format!(
            "{}/payment-sessions/{}/submit",
            self.client.environment.api_url(),
            payment_session_id
        );
        self.client
            .send_post_request("payment-sessions", &url, request)
            .await
    }

    /// Create a payment session and submit a payment attempt in a single
    /// request.
    ///
    /// This request works with the advanced Flow integration, where you do not
    /// need to create a payment session for initializing Flow. You must send
    /// the unmodified response body as the response of the `handleSubmit`
    /// callback.
    ///
    /// [`POST /payment-sessions/submit`](https://api-reference.checkout.com/#operation/CreateAndSubmitPaymentSession)
    pub async fn create_and_submit_payment_session(
        &self,
        request: &CreateAndSubmitPaymentSessionRequest,
    ) -> Result<CreateAndSubmitPaymentSessionResponse, Error> {
        let url = format!(
            "{}/payment-sessions/complete",
            self.client.environment.api_url()
        );
        self.client
            .send_post_request("payment-sessions", &url, request)
            .await
    }
}
