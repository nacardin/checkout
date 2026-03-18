//! Axum server that serves the Checkout.com Flow web component.
//!
//! Reads `CKO_PUBLIC_KEY` from the environment and accepts a
//! `--payment-session` CLI argument (base64-encoded JSON of the full
//! `CreatePaymentSessionResponse`) to dynamically configure the payment page.

use axum::{Router, response::Html, routing::get};
use base64::Engine;
use clap::Parser;
use std::sync::Arc;

/// Serve the Checkout.com Flow payment page.
#[derive(Parser, Debug)]
#[command(name = "flow_frontend")]
struct Args {
    /// Base64-encoded JSON of the full CreatePaymentSessionResponse
    #[arg(long)]
    payment_session: String,

    /// Port to listen on (0 = random available port)
    #[arg(long, default_value_t = 0)]
    port: u16,
}

struct AppState {
    public_key: String,
    /// The full payment session response as a JSON string
    payment_session_json: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let public_key = std::env::var("CKO_PUBLIC_KEY").expect("CKO_PUBLIC_KEY env var must be set");

    // Decode the base64-encoded payment session response
    let session_bytes = base64::engine::general_purpose::STANDARD
        .decode(&args.payment_session)
        .expect("--payment-session must be valid base64");
    // Validate it's valid JSON, then keep as string for injection into the HTML template
    let session: serde_json::Value =
        serde_json::from_slice(&session_bytes).expect("--payment-session must be valid JSON");
    let payment_session_json = session.to_string();

    let state = Arc::new(AppState {
        public_key,
        payment_session_json,
    });

    let app = Router::new().route(
        "/",
        get({
            let state = Arc::clone(&state);
            move || index(state)
        }),
    );

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let actual_port = listener.local_addr().unwrap().port();

    // Machine-readable line — parsed by test harness
    println!("LISTENING_PORT={actual_port}");
    println!("Listening on http://localhost:{actual_port}");
    println!("Payment session: {}", state.payment_session_json);
    println!("Public key: {}", state.public_key);

    axum::serve(listener, app).await.unwrap();
}

async fn index(state: Arc<AppState>) -> Html<String> {
    let html = format!(
        r#"<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Checkout Flow</title>
    <style>
        *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #f5f5f5;
            display: flex;
            justify-content: center;
            align-items: flex-start;
            min-height: 100vh;
            padding: 2rem;
        }}
        .container {{
            background: #fff;
            border-radius: 12px;
            box-shadow: 0 2px 12px rgba(0,0,0,0.08);
            padding: 2rem;
            width: 100%;
            max-width: 480px;
        }}
        .toast {{
            position: fixed; top: 1rem; right: 1rem;
            padding: 0.75rem 1.5rem;
            border-radius: 8px;
            color: #fff;
            font-weight: 600;
            opacity: 0;
            transition: opacity 0.3s;
            pointer-events: none;
            z-index: 100;
        }}
        .toast.success {{ background: #16a34a; }}
        .toast.failed  {{ background: #dc2626; }}
        .toast.show    {{ opacity: 1; pointer-events: auto; }}
    </style>
</head>
<body>
    <div id="successToast" class="toast success">
        <span>The payment was successful</span>
    </div>
    <div id="failedToast" class="toast failed">
        <span>The payment failed, try again</span>
    </div>

    <form>
        <div class="container">
            <div id="flow-container"></div>
        </div>
        <span id="error-message"></span>
    </form>

    <script>
        window._capturedLogs = [];
        const originalError = console.error;
        const originalWarn = console.warn;
        console.error = function(...args) {{ window._capturedLogs.push("ERROR: " + args.join(" ")); originalError.apply(console, args); }};
        console.warn = function(...args) {{ window._capturedLogs.push("WARN: " + args.join(" ")); originalWarn.apply(console, args); }};
        window.addEventListener('unhandledrejection', function(event) {{
            window._capturedLogs.push("REJECTION: " + JSON.stringify(event.reason, Object.getOwnPropertyNames(event.reason)));
        }});
    </script>
    <script src="https://checkout-web-components.checkout.com/index.js"></script>
    <script>
        (async () => {{
            try {{
                console.warn("Starting initialization");
                const publicKey  = '{public_key}';
                const paymentSession = {payment_session_json};
    
                const checkout = await CheckoutWebComponents({{
                    paymentSession,
                    publicKey,
                    environment: 'sandbox',
                }});
                
                console.warn("Checkout component initialized");
    
                const flow = checkout.create('flow');
                flow.mount('#flow-container');
                console.warn("Flow mounted");
            }} catch (err) {{
                console.error("Mount error: " + err.message + "\n" + err.stack);
            }}
        }})();
    </script>
</body>
</html>"#,
        public_key = state.public_key,
        payment_session_json = state.payment_session_json,
    );

    Html(html)
}
