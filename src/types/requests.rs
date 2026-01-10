use super::{
    Amount, BillingDescriptor, Currency, CustomerDescriptor, DestinationInstruction, Metadata,
    PaymentProcessingDescriptor, PaymentRecipient, PaymentRequestDestination, PaymentRequestSource,
    PaymentSenderDetails, PaymentType, RiskRequest, Serialize, ShippingDescriptor, _3DSRequest,
};

/// The request body to be used to authenticate
#[derive(Serialize, Debug, Clone)]
pub struct OAuthTokenRequest {
    /// Probably "`client_credentials`"
    pub grant_type: String,

    /// Determines what endpoints the requested token can access
    ///
    /// See [Authentication](https://api-reference.checkout.com/preview/crusoe/#section/Authentication)
    /// for possible scopes
    pub scope: String,
}

/// Request body for a payment or payout
///
/// To accept payments from cards, digital wallets and many alternative payment
/// methods, specify the `source.type` field, along with the source-specific
/// data.
///
/// To pay out to a card, specify the destination of your payout using the
/// `destination.type` field, along with the destination-specific data.
///
/// See: [Payment Methods](https://docs.checkout.com/payments/payment-methods)
#[derive(Serialize, Debug, Clone)]
pub struct CreatePaymentRequest {
    /// The source of the payment. Use to request a payment.
    pub source: Option<PaymentRequestSource>,

    /// The destination of the payout. Use to pay out to a card.
    pub destination: Option<PaymentRequestDestination>,

    /// The payment amount. The exact format depends on the currency. Omit the
    /// amount or provide a value of 0 to perform a card verification.
    ///
    /// See: [Calculating the value](https://docs.checkout.com/resources/calculating-the-value)
    pub amount: Option<Amount>,

    /// The three-letter ISO country code
    pub currency: Currency,

    /// This must be specified for card payments where the cardholder is not
    /// present (i.e., recurring or mail order / telephone order) (default:
    /// Regular)
    pub payment_type: PaymentType,

    /// Flags the payment as a merchant-initiated transaction (MIT). Must be
    /// set to true for all MITs.
    ///
    /// See: [Requirements for stored payment details](https://docs.checkout.com/payments/store-payment-details/requirements-for-stored-payment-details)
    pub merchant_initiated: bool,

    /// A reference you can later use to identify this payment, such as an
    /// order number. Required when processing via dLocal or Bambora. (<= 50
    /// characters)
    pub reference: Option<String>,

    /// A description of the payment (<= 100 characters)
    pub description: Option<String>,

    /// Whether to capture the payment (if applicable) (default: true)
    pub capture: Option<bool>,

    /// A timestamp (ISO 8601 code) that determines when the payment should be
    /// captured. Providing this field will automatically set capture to true
    pub capture_on: Option<String>,

    /// The customer's details
    pub customer: Option<CustomerDescriptor>,

    /// An optional dynamic billing descriptor displayed on the account owner's
    /// statement
    pub billing_descriptor: Option<BillingDescriptor>,

    /// The shipping details
    pub shipping: Option<ShippingDescriptor>,

    /// Information required for 3D Secure payments
    #[serde(rename = "3ds")]
    pub three_ds: Option<_3DSRequest>,

    /// For payments that use stored card details, such as recurring payments –
    /// an existing payment identifier from the recurring series or the Scheme
    /// Transaction Id (<= 100 characters)
    ///
    /// See: [Requirements for stored payment details](https://docs.checkout.com/payments/store-payment-details/requirements-for-stored-payment-details)
    pub previous_payment_id: Option<String>,

    /// Configures the risk assessment performed during the processing of the
    /// payment
    pub risk: Option<RiskRequest>,

    /// For redirect payment methods, this overrides the default success
    /// redirect URL configured on your account (<= 255 characters)
    pub success_url: Option<String>,

    /// For redirect payment methods, this overrides the default failure
    /// redirect URL configured on your account (<= 255 characters)
    pub failure_url: Option<String>,

    /// The IP address used to make the payment. Required for some risk checks
    /// (<= 45 characters)
    pub payment_ip: Option<String>,

    /// Information about the recipient of the payment's funds. Relevant for
    /// both Account Funding Transactions and VISA or `MasterCard` domestic UK
    /// transactions processed by Financial Institutions.
    ///
    /// See: [Account Funding Transactions](https://docs.checkout.com/payments/manage-payments/account-funding-transactions)
    /// and [Requirements for financial institutions](https://docs.checkout.com/risk-management/requirements-for-financial-institutions)
    pub recipient: Option<PaymentRecipient>,

    /// Use the processing object to influence or override the data sent during
    /// card processing
    pub processing: Option<PaymentProcessingDescriptor>,

    /// The processing channel to be used for the payment
    ///
    /// This can be found under a Payment Method in the Checkout dashboard.
    pub processing_channel_id: String,

    /// Additional details about the payout instruction.
    pub instruction: Option<DestinationInstruction>,

    /// The sender of the payout.
    ///
    /// This field is required for money transfer card payouts.
    pub sender: Option<PaymentSenderDetails>,

    /// Allows you to store additional information about a transaction with
    /// custom fields and up to five user-defined fields (`udf1` to `udf5`),
    /// which can be used for reporting purposes. `udf1` is also used for some
    /// of our risk rules.
    pub metadata: Option<Metadata>,
}

/// A builder for `CreatePaymentRequest`.
#[derive(Debug, Clone)]
pub struct CreatePaymentRequestBuilder {
    source: Option<PaymentRequestSource>,
    destination: Option<PaymentRequestDestination>,
    amount: Option<Amount>,
    currency: Currency,
    payment_type: PaymentType,
    merchant_initiated: bool,
    reference: Option<String>,
    description: Option<String>,
    capture: Option<bool>,
    capture_on: Option<String>,
    customer: Option<CustomerDescriptor>,
    billing_descriptor: Option<BillingDescriptor>,
    shipping: Option<ShippingDescriptor>,
    three_ds: Option<_3DSRequest>,
    previous_payment_id: Option<String>,
    risk: Option<RiskRequest>,
    success_url: Option<String>,
    failure_url: Option<String>,
    payment_ip: Option<String>,
    recipient: Option<PaymentRecipient>,
    processing: Option<PaymentProcessingDescriptor>,
    processing_channel_id: String,
    instruction: Option<DestinationInstruction>,
    sender: Option<PaymentSenderDetails>,
    metadata: Option<Metadata>,
}

impl CreatePaymentRequestBuilder {
    /// Creates a new `CreatePaymentRequestBuilder`.
    #[must_use]
    pub fn new(currency: Currency, processing_channel_id: String) -> Self {
        Self {
            source: None,
            destination: None,
            amount: None,
            currency,
            payment_type: PaymentType::Regular,
            merchant_initiated: false,
            reference: None,
            description: None,
            capture: None,
            capture_on: None,
            customer: None,
            billing_descriptor: None,
            shipping: None,
            three_ds: None,
            previous_payment_id: None,
            risk: None,
            success_url: None,
            failure_url: None,
            payment_ip: None,
            recipient: None,
            processing: None,
            processing_channel_id,
            instruction: None,
            sender: None,
            metadata: None,
        }
    }

    /// Sets the source of the payment.
    #[must_use]
    pub fn source(mut self, source: PaymentRequestSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the destination of the payment.
    #[must_use]
    pub fn destination(mut self, destination: PaymentRequestDestination) -> Self {
        self.destination = Some(destination);
        self
    }

    /// Sets the amount of the payment.
    #[must_use]
    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets the payment type.
    #[must_use]
    pub fn payment_type(mut self, payment_type: PaymentType) -> Self {
        self.payment_type = payment_type;
        self
    }

    /// Sets whether the payment is merchant-initiated.
    #[must_use]
    pub fn merchant_initiated(mut self, merchant_initiated: bool) -> Self {
        self.merchant_initiated = merchant_initiated;
        self
    }

    /// Sets the reference for the payment.
    #[must_use]
    pub fn reference(mut self, reference: String) -> Self {
        self.reference = Some(reference);
        self
    }

    /// Sets the description for the payment.
    #[must_use]
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets whether to capture the payment.
    #[must_use]
    pub fn capture(mut self, capture: bool) -> Self {
        self.capture = Some(capture);
        self
    }

    /// Sets the capture timestamp for the payment.
    #[must_use]
    pub fn capture_on(mut self, capture_on: String) -> Self {
        self.capture_on = Some(capture_on);
        self
    }

    /// Sets the customer for the payment.
    #[must_use]
    pub fn customer(mut self, customer: CustomerDescriptor) -> Self {
        self.customer = Some(customer);
        self
    }

    /// Sets the billing descriptor for the payment.
    #[must_use]
    pub fn billing_descriptor(mut self, billing_descriptor: BillingDescriptor) -> Self {
        self.billing_descriptor = Some(billing_descriptor);
        self
    }

    /// Sets the shipping details for the payment.
    #[must_use]
    pub fn shipping(mut self, shipping: ShippingDescriptor) -> Self {
        self.shipping = Some(shipping);
        self
    }

    /// Sets the 3D Secure details for the payment.
    #[must_use]
    pub fn three_ds(mut self, three_ds: _3DSRequest) -> Self {
        self.three_ds = Some(three_ds);
        self
    }

    /// Sets the previous payment ID for the payment.
    #[must_use]
    pub fn previous_payment_id(mut self, previous_payment_id: String) -> Self {
        self.previous_payment_id = Some(previous_payment_id);
        self
    }

    /// Sets the risk request for the payment.
    #[must_use]
    pub fn risk(mut self, risk: RiskRequest) -> Self {
        self.risk = Some(risk);
        self
    }

    /// Sets the success URL for the payment.
    #[must_use]
    pub fn success_url(mut self, success_url: String) -> Self {
        self.success_url = Some(success_url);
        self
    }

    /// Sets the failure URL for the payment.
    #[must_use]
    pub fn failure_url(mut self, failure_url: String) -> Self {
        self.failure_url = Some(failure_url);
        self
    }

    /// Sets the payment IP for the payment.
    #[must_use]
    pub fn payment_ip(mut self, payment_ip: String) -> Self {
        self.payment_ip = Some(payment_ip);
        self
    }

    /// Sets the recipient for the payment.
    #[must_use]
    pub fn recipient(mut self, recipient: PaymentRecipient) -> Self {
        self.recipient = Some(recipient);
        self
    }

    /// Sets the processing descriptor for the payment.
    #[must_use]
    pub fn processing(mut self, processing: PaymentProcessingDescriptor) -> Self {
        self.processing = Some(processing);
        self
    }

    /// Sets the instruction for the payment.
    #[must_use]
    pub fn instruction(mut self, instruction: DestinationInstruction) -> Self {
        self.instruction = Some(instruction);
        self
    }

    /// Sets the sender details for the payment.
    #[must_use]
    pub fn sender(mut self, sender: PaymentSenderDetails) -> Self {
        self.sender = Some(sender);
        self
    }

    /// Sets the metadata for the payment.
    #[must_use]
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Builds the `CreatePaymentRequest`.
    #[must_use]
    pub fn build(self) -> CreatePaymentRequest {
        CreatePaymentRequest {
            source: self.source,
            destination: self.destination,
            amount: self.amount,
            currency: self.currency,
            payment_type: self.payment_type,
            merchant_initiated: self.merchant_initiated,
            reference: self.reference,
            description: self.description,
            capture: self.capture,
            capture_on: self.capture_on,
            customer: self.customer,
            billing_descriptor: self.billing_descriptor,
            shipping: self.shipping,
            three_ds: self.three_ds,
            previous_payment_id: self.previous_payment_id,
            risk: self.risk,
            success_url: self.success_url,
            failure_url: self.failure_url,
            payment_ip: self.payment_ip,
            recipient: self.recipient,
            processing: self.processing,
            processing_channel_id: self.processing_channel_id,
            instruction: self.instruction,
            sender: self.sender,
            metadata: self.metadata,
        }
    }
}

/// Body used in the request to capture a payment
#[derive(Serialize, Debug, Clone)]
pub struct CapturePaymentBody {
    /// The amount to capture. If not specified, the full payment amount will
    /// be captured
    pub amount: Option<u64>,

    /// A reference you can later use to identify this capture request
    pub reference: Option<String>,

    /// A set of key-value pairs that you can attach to the capture request.
    /// This can be useful for storing additional information in a structured
    /// format
    pub metadata: Option<Metadata>,
}

/// Body used in the request to refund a payment
#[derive(Serialize, Debug, Clone)]
pub struct RefundPaymentBody {
    /// The amount to refund. If not specified, the full payment amount will
    /// be refunded
    pub amount: Option<u64>,

    /// A reference you can later use to identify this refund request
    pub reference: Option<String>,

    /// A set of key-value pairs that you can attach to the refund request.
    /// This can be useful for storing additional information in a structured
    /// format
    pub metadata: Option<Metadata>,
}

/// Request body for creating a payment session
#[derive(Serialize, Debug, Clone)]
pub struct CreatePaymentSessionRequest {
    /// The payment amount
    pub amount: u64,

    /// The three-letter ISO currency code
    pub currency: Currency,

    /// A reference you can later use to identify this payment session
    pub reference: String,

    /// The billing details
    pub billing: Option<BillingDescriptor>,

    /// The customer's details
    pub customer: Option<CustomerDescriptor>,

    /// The URL to redirect to if the payment is successful
    pub success_url: String,

    /// The URL to redirect to if the payment fails
    pub failure_url: String,
}

/// Body used in the request to void a payment
#[derive(Serialize, Debug, Clone)]
pub struct VoidPaymentBody {
    /// A reference you can later use to identify this void request
    pub reference: Option<String>,

    /// A set of key-value pairs that you can attach to the void request.
    /// This can be useful for storing additional information in a structured
    /// format
    pub metadata: Option<Metadata>,
}

/// Request body to create an instrument
#[derive(Serialize, Debug, Clone)]
pub struct CreateInstrumentBody {
    /// The instrument type
    #[serde(rename = "type")]
    ty: String,
}
