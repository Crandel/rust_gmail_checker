use serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    account: String,
    short: String,
    mail: String,
    password: String,
}

impl Account {
    pub fn new(account: String, short: String, mail: String, password: String,) -> Account {
        Account {
            account: account,
            short: short,
            mail: mail,
            password: password,
        }
    }
}
