extern crate base64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmailType {
    Gmail,
    ProtonMail,
}

pub trait ServiceUrl {
    fn get_url(&self) -> &str;
}
#[derive(Debug)]
pub struct Basic {
    username: String,
    password: String,
}

impl Basic {
    pub fn new(username: String, password: String) -> Basic {
        Basic {
            username: username,
            password: password,
        }
    }
    pub fn encode_tostr(&self) -> String {
        let mut auth_str = String::from("Basic ");
        let user_data = format!("{}:{}", self.username, self.password);
        let b64 = base64::encode(user_data.as_bytes());
        auth_str.push_str(b64.as_str());
        auth_str
    }
}
