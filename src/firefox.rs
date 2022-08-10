use crate::OS;
use chrono::{NaiveDate, Utc};

pub async fn get_latest_ua(os: OS) -> Option<String> {
    let body: String = reqwest::get("https://www.mozilla.org/en-US/firefox/new/")
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    let attr = "data-latest-firefox=\"";
    if let Some(mut start_index) = body.find(attr) {
        start_index += attr.len();
        if let Some(length) = body[start_index..].find("\"") {
            let version = &body[start_index..start_index + length];

            let numbers: Vec<_> = version.split(".").into_iter().take(2).collect();
            let version = numbers.join(".");

            return Some(format_version(&version, os));
        }
    }

    None
}

pub fn estimate_latest_ua(os: OS) -> String {
    let known_version = 103;
    let known_date = NaiveDate::from_ymd(2022, 07, 26);
    let versions_per_year = 12;

    let days_between_version = (365 + (versions_per_year - 1)) / versions_per_year;

    let duration_days = (Utc::now().date_naive() - known_date).num_days();
    let version = known_version + (duration_days / days_between_version);

    format_version(&format!("{version}.0"), os)
}

pub fn format_version(version: &str, os: OS) -> String {
    match os {
        OS::LinuxIntelx64 =>
            format!("Mozilla/5.0 (X11; Linux x86_64; rv:{version}) Gecko/20100101 Firefox/{version}"),
        OS::WindowsIntelx64 =>
            format!("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:{version}) Gecko/20100101 Firefox/{version}"),
    }
}
