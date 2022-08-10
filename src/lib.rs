mod firefox;

#[derive(Debug, Clone)]
pub struct UserAgent {
    pub user_agent: String,
    pub is_estimated: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Browser {
    Firefox,
}

#[derive(Debug, Copy, Clone)]
pub enum OS {
    LinuxIntelx64,
    WindowsIntelx64,
}

/// Gets latest user agent for specified Browser and Operating System.
///
/// It never fails. If the Internet connection fails it logs the fact and
/// estimates the agent string offline.
pub async fn get_latest_user_agent(browser: Browser, os: OS, offline_mode: bool) -> UserAgent {
    let mut version = None;
    let mut is_estimated = false;

    if !offline_mode {
        version = match browser {
            Browser::Firefox => firefox::get_latest_ua(os).await,
        };
    }

    if version.is_none() {
        if !offline_mode {
            log::warn!("Couldn't retrieve actual UserAgent for {:?} from internet. UserAgent will be estimated.", browser);
        }

        version = Some(match browser {
            Browser::Firefox => firefox::estimate_latest_ua(os),
        });
        is_estimated = true;
    }

    UserAgent {
        user_agent: version.unwrap(),
        is_estimated,
    }
}
