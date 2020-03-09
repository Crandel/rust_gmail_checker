use base64;

pub trait Extractor {
    fn get_url(&self) -> &str;
    fn extract_result(&self, body_res: String) -> String;
}

pub fn encode_tostr(username: &str, password: &str) -> String {
    let user_data = format!("{}:{}", username, password);
    let b64 = base64::encode(user_data.as_bytes());
    format!("Basic {}", b64)
}
