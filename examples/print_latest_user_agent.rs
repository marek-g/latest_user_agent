use latest_user_agent::{get_latest_user_agent, Browser, OS};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    println!(
        "Latest Firefox User Agent: {:?}",
        get_latest_user_agent(Browser::Firefox, OS::LinuxIntelx64, false).await
    )
}
