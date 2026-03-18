#[tokio::test]
async fn debug_page() {
    let opts = headless_chrome::LaunchOptions::default_builder()
        .headless(true)
        .build()
        .unwrap();
    let browser = headless_chrome::Browser::new(opts).unwrap();
    let tab = browser.new_tab().unwrap();

    let _ = tab.navigate_to("http://127.0.0.1:34365");
    tab.wait_until_navigated().unwrap();
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    
    let html = tab.get_content().unwrap();
    println!("HTML length: {}", html.len());
    
    // Instead of raw event logs, let's inject a script that catches errors.
    // Actually, just reading page content might sometimes indicate what failed.
}
