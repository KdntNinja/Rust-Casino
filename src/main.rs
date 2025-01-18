use thirtyfour::prelude::*;
use std::process::{Command, Child};
use tokio::runtime::Runtime;
use tokio::time::sleep;
use std::time::Duration;

struct WebDriverSession {
    driver: WebDriver,
    geckodriver_process: Option<Child>,
}

impl WebDriverSession {
    // Initializes a new WebDriver session
    async fn new() -> WebDriverResult<Self> {
        let geckodriver_process = start_geckodriver();

        // Use tokio's async sleep instead of std::thread::sleep
        sleep(Duration::from_secs(1)).await;

        let caps = DesiredCapabilities::firefox();
        let driver = WebDriver::new("http://localhost:4444", caps).await?;

        Ok(Self {
            driver,
            geckodriver_process,
        })
    }

    // Quit the WebDriver and stop the geckodriver process
    async fn quit(self) -> WebDriverResult<()> {
        self.driver.quit().await?;

        if let Some(mut process) = self.geckodriver_process {
            process.kill().expect("Failed to kill geckodriver");
        }

        Ok(())
    }
}

fn start_geckodriver() -> Option<Child> {
    Command::new("geckodriver")
        .arg("--port")
        .arg("4444")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .ok()
}

fn main() {
    // Create a runtime and run the async WebDriver session
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    runtime.block_on(async {
        let session = WebDriverSession::new().await.unwrap();

        // Your automation logic goes here, for example:
        session.driver.goto("https://wikipedia.org").await.unwrap();
        let elem_form = session.driver.find(By::Id("search-form")).await.unwrap();

        let elem_text = elem_form.find(By::Id("searchInput")).await.unwrap();
        elem_text.send_keys("selenium").await.unwrap();

        let elem_button = elem_form.find(By::Css("button[type='submit']")).await.unwrap();
        elem_button.click().await.unwrap();

        session.quit().await.unwrap();
    });
}
