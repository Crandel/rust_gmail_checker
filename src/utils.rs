use crate::accounts::Account;
use base64;
use hyper::header::HeaderMap;

pub trait ServiceUrl {
    fn get_url(&self) -> &str;
    fn extract_result(&self, body_res: String) -> String;
    fn create_headers(&self, acc: &Account) -> HeaderMap;
}

#[derive(Debug)]
pub struct Basic {
    username: String,
    password: String,
}

impl Basic {
    pub fn new(username: String, password: String) -> Basic {
        Basic { username, password }
    }
    pub fn encode_tostr(&self) -> String {
        let mut auth_str = String::from("Basic ");
        let user_data = format!("{}:{}", self.username, self.password);
        let b64 = base64::encode(user_data.as_bytes());
        auth_str.push_str(b64.as_str());
        auth_str
    }
}
