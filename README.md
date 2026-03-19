# Checkout.com Rust SDK

An unofficial Rust client for the [Checkout.com API](https://www.checkout.com/docs/api) powered by `reqwest`.

## Supported APIs

-   Payment Session (Flow)
-   Payment

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
checkout = "0.4.0"
```

### Creating a Client

You can create a client by providing your API keys and the desired environment:

```rust
use checkout::{Client, Environment};
use secrecy::SecretString;

let client = Client::new(
    SecretString::new("YOUR_USERNAME".to_string()),
    SecretString::new("YOUR_PASSWORD".to_string()),
    Environment::Sandbox,
);
```

Alternatively, you can create a client from environment variables:

-   `CKO_USERNAME`
-   `CKO_PASSWORD`
-   `CKO_ENVIRONMENT` (`sandbox` or `production`)

```rust
let client = Client::from_env().unwrap();
```

### Creating a Payment Session (Flow)

```rust
use checkout::{CreatePaymentSessionRequest, Currency};

let request = CreatePaymentSessionRequest {
    amount: 1000,
    currency: Currency::USD,
    reference: "ORD-123A".to_string(),
    billing: None,
    customer: None,
    success_url: "https://example.com/success".to_string(),
    failure_url: "https://example.com/failure".to_string(),
};

let response = client.create_payment_session(&request).await.unwrap();
```

### Creating a Payment

```rust
use checkout::{CreatePaymentRequest, Currency, PaymentRequestSource, Amount};
use bigdecimal::BigDecimal;

let request = CreatePaymentRequest::builder()
    .currency(Currency::USD)
    .processing_channel_id("YOUR_PROCESSING_CHANNEL_ID".to_string())
    .source(PaymentRequestSource::Card {
        number: "4242424242424242".to_string(),
        expiry_month: 12,
        expiry_year: 2025,
        name: Some("Test User".to_string()),
        cvv: Some("123".to_string()),
        stored: None,
        billing_address: Box::new(None),
        phone: None,
    })
    .amount(Amount::from(Currency::USD, BigDecimal::from(1000)))
    .build();

let response = client.create_payment(&request).await.unwrap();
```

## Testing

All tests hit the **Checkout.com Sandbox** and require valid API credentials.

### Environment Variables

Create a `.env` file (git-ignored) with:

```env
CKO_USERNAME=<OAuth client ID>
CKO_PASSWORD=<OAuth client secret>
CKO_ENVIRONMENT=sandbox
CKO_PROCESSING_CHANNEL_ID=<sandbox processing channel>
CKO_PUBLIC_KEY=<sandbox public key>          # E2E tests only
CKO_CURRENCY_ACCOUNT_ID=<currency account>   # payout tests only
```

### Test Dependencies

| Crate | Purpose |
|---|---|
| `tokio` | Async test runtime |
| `dotenvy` | Load `.env` credentials |
| `axum` | Inline HTTP server for E2E flow pages |
| `headless_chrome` | Browser automation for E2E tests |
| `rand` | Unique payment references |
| `base64` | Encoding helpers |

### Running Tests

```bash
# Integration tests (API-level, no browser)
cargo test --test flows --test payments --test metadata

# E2E tests (requires headless Chrome)
cargo test --test flows_e2e -- --ignored

# All tests
cargo test -- --include-ignored
```

> [!NOTE]
> Tests that depend on missing environment variables silently return early rather than failing, so a partial `.env` will skip rather than break the suite.
