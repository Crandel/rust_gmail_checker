use serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    account: String,
    short_conky: String,
    email: String,
    password: String,
}

impl Account {
    pub fn new(account: String, short: String, email: String, password: String,) -> Account {
        Account {
            account: account,
            short_conky: short,
            email: email,
            password: password,
        }
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_short(&self) -> &str {
        &self.short_conky
    }
}
