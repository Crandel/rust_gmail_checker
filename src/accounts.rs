use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    mail_type: EmailType,
    account: String,
    short_alias: String,
    client_id: String,
    client_secret: String,
}

impl Account {
    // Account constructor
    pub fn new(
        mail_type: EmailType,
        account: String,
        short_alias: String,
        client_id: String,
        client_secret: String,
    ) -> Account {
        Account {
            mail_type,
            account,
            short_alias,
            client_id,
            client_secret,
        }
    }

    // public getter for email
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    // public getter for password
    pub fn get_password(&self) -> &str {
        &self.client_secret
    }

    // public getter for short_conky value
    pub fn get_short(&self) -> &str {
        &self.short_alias
    }
    // public getter for short_conky value
    pub fn get_mail_type(&self) -> EmailType {
        self.mail_type
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EmailType {
    Gmail,
    ProtonMail,
}

#[cfg(test)]
mod tests {
    use super::Account;
    use super::EmailType;
    #[test]
    fn acc_test() {
        let mail_type = EmailType::Gmail;
        let name = "test_name";
        let client_id = "test_email";
        let client_secret = "test_password";
        let short = "test_short";
        let acc = Account::new(
            mail_type,
            String::from(name),
            String::from(short),
            String::from(client_id),
            String::from(client_secret),
        );

        assert_eq!(mail_type, acc.get_mail_type());
        assert_eq!(client_id, acc.get_client_id());
        assert_eq!(client_secret, acc.get_password());
        assert_eq!(short, acc.get_short());
    }
}
