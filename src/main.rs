use headless_chrome::protocol::cdp::Page;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};

fn browse_wikipedia() -> Result<(), failure::Error> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    /// Navigate to wikipedia
    tab.navigate_to("https://www.wikipedia.org")?;

    Ok(())
}

assert!(browse_wikipedia().is_ok());
