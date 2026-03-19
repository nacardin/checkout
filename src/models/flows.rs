use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::models::shared::{
    BillingDescriptor, BillingInformation, Currency, CustomerDescriptor, DestinationInstruction,
    Links, Metadata, PaymentRecipient, PaymentSenderDetails, PaymentType, RiskRequest,
    ShippingDescriptor, _3DSRequest,
};

// ---------------------------------------------------------------------------
// Create Payment Session
// ---------------------------------------------------------------------------

/// Request body for creating a payment session
///
/// [`POST /payment-sessions`](https://api-reference.checkout.com/#operation/CreatePaymentSession)
#[derive(Serialize, Debug, Clone, Builder)]
pub struct CreatePaymentSessionRequest {
    /// The payment amount
    pub amount: u64,

    /// The three-letter ISO currency code
    pub currency: Currency,

    /// The processing channel to be used for the payment
    #[builder(into)]
    pub processing_channel_id: String,

    /// A reference you can later use to identify this payment session
    #[builder(into)]
    pub reference: String,

    /// The billing details
    pub billing: BillingInformation,

    /// The customer's details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerDescriptor>,

    /// The URL to redirect to if the payment is successful
    #[builder(into)]
    pub success_url: String,

    /// The URL to redirect to if the payment fails
    #[builder(into)]
    pub failure_url: String,

    /// The payment type (e.g., Regular, Recurring, MOTO)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,

    /// A description of the purchase, displayed on the customer's statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_descriptor: Option<BillingDescriptor>,

    /// A description for the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    /// The shipping details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDescriptor>,

    /// Information about the recipient of the payment's funds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<PaymentRecipient>,

    /// Use the processing object to influence or override the data sent during
    /// card processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<serde_json::Value>,

    /// Details about the payment instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<DestinationInstruction>,

    /// The line items in the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,

    /// The sub-entities that the payment is being processed on behalf of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_allocations: Option<Vec<serde_json::Value>>,

    /// Configures the risk assessment performed during payment processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<RiskRequest>,

    /// The merchant's display name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub display_name: Option<String>,

    /// Allows you to store additional information about a transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Creates a translated version of the page in the specified language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub locale: Option<String>,

    /// Information required for 3D Secure authentication payments
    #[serde(rename = "3ds", skip_serializing_if = "Option::is_none")]
    pub three_ds: Option<_3DSRequest>,

    /// The sender of the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<PaymentSenderDetails>,

    /// Specifies whether to capture the payment, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    /// A timestamp specifying when to capture the payment (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub capture_on: Option<String>,

    /// A timestamp specifying when the PaymentSession should expire (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub expires_on: Option<String>,

    /// Specifies which payment method options to present to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_payment_methods: Option<Vec<String>>,

    /// Specifies which payment method options to not present to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_payment_methods: Option<Vec<String>>,

    /// Configurations for payment method-specific settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<serde_json::Value>,

    /// Configuration for asynchronous retries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_retry: Option<serde_json::Value>,

    /// The Customer's IP address (IPv4 or IPv6)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ip_address: Option<String>,
}

/// Response for creating a payment session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatePaymentSessionResponse {
    /// The payment session identifier
    pub id: String,

    /// A unique token representing the payment session, used to initialize Flow
    pub payment_session_token: String,

    /// The payment session secret
    pub payment_session_secret: String,

    /// The links related to the payment session
    #[serde(rename = "_links")]
    pub links: Links,
}

// ---------------------------------------------------------------------------
// Submit Payment Session
// ---------------------------------------------------------------------------

/// Request body for submitting a payment attempt for an existing payment session.
///
/// [`PUT /payment-sessions/{id}/submit`](https://api-reference.checkout.com/#operation/SubmitPaymentSession)
#[derive(Serialize, Debug, Clone, Builder)]
pub struct SubmitPaymentSessionRequest {
    /// A unique token representing the additional customer data captured by
    /// Flow, as received from the handleSubmit callback. Do not log or store
    /// this value.
    #[builder(into)]
    pub session_data: String,

    /// The payment amount override (minor currency unit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,

    /// Overrides the default success redirect URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub success_url: Option<String>,

    /// Overrides the default failure redirect URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub failure_url: Option<String>,

    /// The payment type (e.g., Regular, Recurring, MOTO)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,

    /// A description of the purchase, displayed on the customer's statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_descriptor: Option<BillingDescriptor>,

    /// A reference you can use to identify the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub reference: Option<String>,

    /// The line items in the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,

    /// Information required for 3D Secure authentication payments
    #[serde(rename = "3ds", skip_serializing_if = "Option::is_none")]
    pub three_ds: Option<_3DSRequest>,

    /// The Customer's IP address (IPv4 or IPv6)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub ip_address: Option<String>,

    /// Configurations for payment method-specific settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<serde_json::Value>,

    /// Information about the recipient of the payment's funds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<PaymentRecipient>,

    /// Details about the payment instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<DestinationInstruction>,

    /// The sender of the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<PaymentSenderDetails>,

    /// Specifies whether to capture the payment, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    /// A timestamp specifying when to capture the payment (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub capture_on: Option<String>,
}

/// Response for submitting a payment session.
///
/// Returned for both 201 (processed) and 202 (async/redirect) responses.
#[derive(Deserialize, Debug, Clone)]
pub struct SubmitPaymentSessionResponse {
    /// The payment's unique identifier (format: `pay_*`)
    pub id: String,

    /// The status of the payment (e.g., "Approved", "Pending")
    pub status: String,

    /// The payment method type used (e.g., "card", "applepay", "alipay_cn")
    #[serde(rename = "type")]
    pub payment_type: Option<String>,
}

// ---------------------------------------------------------------------------
// Create and Submit Payment Session
// ---------------------------------------------------------------------------

/// Request body for creating a payment session and submitting a payment attempt
/// in a single request.
///
/// [`POST /payment-sessions/submit`](https://api-reference.checkout.com/#operation/CreateAndSubmitPaymentSession)
#[derive(Serialize, Debug, Clone, Builder)]
pub struct CreateAndSubmitPaymentSessionRequest {
    /// A unique token representing the additional customer data captured by
    /// Flow, as received from the handleSubmit callback. Do not log or store
    /// this value.
    #[builder(into)]
    pub session_data: String,

    /// The payment amount (minor currency unit)
    pub amount: u64,

    /// The three-letter ISO currency code
    pub currency: Currency,

    /// The processing channel to be used for the payment
    #[builder(into)]
    pub processing_channel_id: String,

    /// A reference you can later use to identify this payment
    #[builder(into)]
    pub reference: String,

    /// The billing details
    pub billing: BillingInformation,

    /// The customer's details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerDescriptor>,

    /// The URL to redirect to if the payment is successful
    #[builder(into)]
    pub success_url: String,

    /// The URL to redirect to if the payment fails
    #[builder(into)]
    pub failure_url: String,

    /// The payment type (e.g., Regular, Recurring, MOTO)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,

    /// A description of the purchase, displayed on the customer's statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_descriptor: Option<BillingDescriptor>,

    /// A description for the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub description: Option<String>,

    /// The shipping details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDescriptor>,

    /// Information about the recipient of the payment's funds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<PaymentRecipient>,

    /// Use the processing object to influence or override the data sent during
    /// card processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<serde_json::Value>,

    /// Details about the payment instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<DestinationInstruction>,

    /// The line items in the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<serde_json::Value>>,

    /// The sub-entities that the payment is being processed on behalf of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_allocations: Option<Vec<serde_json::Value>>,

    /// Configures the risk assessment performed during payment processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk: Option<RiskRequest>,

    /// The merchant's display name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub display_name: Option<String>,

    /// Allows you to store additional information about a transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Creates a translated version of the page in the specified language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub locale: Option<String>,

    /// Information required for 3D Secure authentication payments
    #[serde(rename = "3ds", skip_serializing_if = "Option::is_none")]
    pub three_ds: Option<_3DSRequest>,

    /// The sender of the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<PaymentSenderDetails>,

    /// Specifies whether to capture the payment, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    /// A timestamp specifying when to capture the payment (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub capture_on: Option<String>,
}

/// Response for creating and submitting a payment session in a single request.
///
/// Returned for both 201 (processed) and 202 (async/redirect) responses.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateAndSubmitPaymentSessionResponse {
    /// The payment's unique identifier (format: `pay_*`)
    pub id: String,

    /// The status of the payment (e.g., "Approved", "Pending")
    pub status: String,

    /// The payment method type used (e.g., "card", "applepay", "alipay_cn")
    #[serde(rename = "type")]
    pub payment_type: Option<String>,

    /// The payment session identifier (format: `ps_*`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_session_id: Option<String>,

    /// The payment session secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_session_secret: Option<String>,
}
