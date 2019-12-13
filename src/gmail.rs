use crate::utils::{ServiceUrl, Result};
use regex::Regex;

pub struct GmailHandler {
    fullcount: Regex,
    count_number: Regex,
}

impl Default for GmailHandler {
    fn default() -> GmailHandler {
        let count_number: Regex = Regex::new("[0-9]+").unwrap();
        let fullcount: Regex = Regex::new("<fullcount>(.*?)</fullcount>").unwrap();
        GmailHandler {
            fullcount,
            count_number,
        }
    }
}

impl ServiceUrl for GmailHandler {
    fn get_url(&self) -> &str {
        "https://mail.google.com/mail/feed/atom"
    }

    fn extract_result(&self, body_res: String) -> Result {
        if let Some(count) = self.fullcount.find(body_res.as_str()) {
            let fullcount_str = count.as_str();
            if let Some(res) = self.count_number.find(fullcount_str) {
                Result::Success(String::from(res.as_str()))
            } else { Result::Failure }
        } else { Result::Failure }
    }
}
