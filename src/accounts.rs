use serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    account: String,
    short_conky: String,
    email: String,
    password: String,
}

impl Account {
    pub fn new(account: String, short: String, mail: String, password: String,) -> Account {
        Account {
            account: account,
            short_conky: short,
            email: mail,
            password: password,
        }
    }
}
