# Testing the Checkout SDK

This SDK includes end-to-end (E2E) tests that verify the complete payment flow using a simulated web browser. We use **`headless_chrome`** to provide a direct, truly zero-setup testing experience using your system's existing Chrome installation.

## Running End-to-End Tests natively

Unlike older WebDriver-based testing frameworks (like Selenium or Fantoccini), you **do NOT** need to install or run any external servers like `geckodriver` or `chromedriver`. 

To run the full E2E test suite locally, follow these simple steps:

### 1. Variables Setup

Ensure that you have populated your `.env` file with the client identifier keys required by the Checkout Web Components and Server side:

```env
CKO_PROCESSING_CHANNEL_ID="pc_..."
CKO_PUBLIC_KEY="pk_sbox_..."
```

*(Note: `CKO_PUBLIC_KEY` is specifically required to initialize the client-side Flow iframe interface correctly during the E2E test.)*

### 2. Run the Tests

You can simply execute the test suite normally. The tests will seamlessly locate your system's Chrome installation, spawn it headlessly, set up a temporary HTTP sandbox, and execute the payment interface flow dynamically. 

```bash
# Run all tests
cargo test

# Or specifically test the E2E flow and watch the logs
cargo test payment_session_request_processed_e2e -- --nocapture
```
