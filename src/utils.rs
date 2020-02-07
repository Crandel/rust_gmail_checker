use base64;

pub trait ServiceUrl {
    fn get_url(&self) -> &str;
    fn extract_result(&self, body_res: String) -> String;
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
        let user_data = format!("{}:{}", self.username, self.password);
        let b64 = base64::encode(user_data.as_bytes());
        format!("Basic {}", b64)
    }
}
