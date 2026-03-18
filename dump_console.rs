use headless_chrome::Browser;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.enable_log()?;
    
    let logs = Arc::new(std::sync::Mutex::new(Vec::new()));
    let logs_clone = logs.clone();
    
    tab.add_event_listener(Arc::new(move |event| {
        if let headless_chrome::protocol::cdp::Event::Log(log_event) = event {
            if let headless_chrome::protocol::cdp::log::Event::EntryAdded(entry) = log_event {
                println!("CONSOLE: {:?}", entry.entry);
                logs_clone.lock().unwrap().push(entry.entry.clone());
            }
        }
    }))?;

    tab.navigate_to("http://127.0.0.1:34365")?;
    tab.wait_until_navigated()?;
    
    std::thread::sleep(std::time::Duration::from_secs(3));
    Ok(())
}
