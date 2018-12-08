use crate::utils::EmailType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    mail_type: EmailType,
    account: String,
    short_conky: String,
    email: String,
    password: String,
}

impl Account {
    // Account constructor
    pub fn new(
        mail_type: EmailType,
        account: String,
        short_conky: String,
        email: String,
        password: String,
    ) -> Account {
        Account {
            mail_type,
            account,
            short_conky,
            email,
            password,
        }
    }

    // public getter for email
    pub fn get_email(&self) -> &str {
        &self.email
    }

    // public getter for password
    pub fn get_password(&self) -> &str {
        &self.password
    }

    // public getter for short_conky value
    pub fn get_short(&self) -> &str {
        &self.short_conky
    }
    // public getter for short_conky value
    pub fn get_mail_type(&self) -> EmailType {
        self.mail_type
    }
}

#[cfg(test)]
mod tests {
    use super::Account;
    use crate::utils::EmailType;
    #[test]
    fn acc_test() {
        let mail_type = EmailType::Gmail;
        let name = "test_name";
        let email = "test_email";
        let password = "test_password";
        let short = "test_short";
        let acc = Account::new(
            mail_type,
            String::from(name),
            String::from(short),
            String::from(email),
            String::from(password),
        );

        assert_eq!(mail_type, acc.get_mail_type());
        assert_eq!(email, acc.get_email());
        assert_eq!(password, acc.get_password());
        assert_eq!(short, acc.get_short());
    }
}
