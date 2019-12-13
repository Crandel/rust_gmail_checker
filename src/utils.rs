#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmailType {
    Gmail,
    ProtonMail,
}

pub trait ServiceUrl {
    fn get_url(&self) -> &str;
    fn extract_result(&self, body_res: String) -> Result;
}

pub enum Result {
    Success(String),
    Failure
}
