use accounts::Account;
use hyper::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use regex::Regex;
use utils::{Basic, ServiceUrl};

pub struct GmailHandler {
    fullcount: Regex,
    count_number: Regex,
}

impl GmailHandler {
    pub fn new() -> GmailHandler {
        let count_number: Regex = Regex::new("[0-9]+").unwrap();
        let fullcount: Regex = Regex::new("<fullcount>(.*?)</fullcount>").unwrap();
        GmailHandler {
            fullcount: fullcount,
            count_number: count_number,
        }
    }
}

impl ServiceUrl for GmailHandler {
    fn get_url(&self) -> &str {
        "https://mail.google.com/mail/feed/atom"
    }

    fn extract_result(&self, body_str: String) -> String {
        let str_body = body_str.as_str();
        let result = match self.fullcount.find(str_body) {
            Some(count) => {
                let fullcount_str = &str_body[count.start()..count.end()];
                match self.count_number.find(fullcount_str) {
                    Some(res) => &fullcount_str[res.start()..res.end()],
                    None => "",
                }
            }
            None => "",
        };
        String::from(result)
    }

    fn create_headers(&self, acc: &Account) -> HeaderMap {
        let basic = Basic::new(
            String::from(acc.get_email()),
            String::from(acc.get_password()),
        );
        let base_str = basic.encode_tostr();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(base_str.as_str()).unwrap(),
        );
        headers
    }
}
