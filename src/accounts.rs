use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    mail_type: EmailType,
    account: String,
    short_alias: String,
    client_id: String,
    client_secret: Option<String>,
    refresh_token: Option<String>,
}

impl Account {
    // Account constructor
    pub fn new(
        mail_type: EmailType,
        account: String,
        short_alias: String,
        client_id: String,
    ) -> Self {
        let secret: Option<String> = None;
        let token: Option<String> = None;
        Account {
            mail_type,
            account,
            short_alias,
            client_id,
            client_secret: secret,
            refresh_token: token,
        }
    }

    // public getter for email
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    // public getter for client secret
    pub fn get_client_secret(&self) -> Option<String> {
        if let Some(secret) = self.client_secret.as_deref() {
            return Some(String::from(secret));
        }
        return None;
    }

    // public setter for client secret
    pub fn set_client_secret(&mut self, secret: String) {
        if secret != "" {
            self.client_secret = Some(secret)
        }
    }

    // public getter for refresh token
    pub fn get_refresh_token(&self) -> Option<String> {
        if let Some(token) = self.refresh_token.as_deref() {
            return Some(String::from(token));
        }
        return None;
    }

    // public setter for refresh token
    pub fn set_refresh_token(&mut self, token: String) {
        if token != "" {
            self.refresh_token = Some(token)
        }
    }

    // public getter for short alias value
    pub fn get_short(&self) -> &str {
        &self.short_alias
    }
    // public getter for mail type value
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
        let client_id = "test_id";
        let short = "test_short";
        let def_client_secret: Option<String> = None;
        let mut acc = Account::new(
            mail_type,
            String::from(name),
            String::from(short),
            String::from(client_id),
        );

        assert_eq!(mail_type, acc.get_mail_type());
        assert_eq!(client_id, acc.get_client_id());
        assert_eq!(def_client_secret, acc.get_client_secret());
        assert_eq!(short, acc.get_short());

        let client_secret_str = "test secret";
        let client_secret: Option<String> = Some(String::from(client_secret_str));
        let refresh_token_str = "test secret";
        let refresh_token: Option<String> = Some(String::from(refresh_token_str));
        acc.set_client_secret(String::from(client_secret_str));
        acc.set_refresh_token(String::from(refresh_token_str));
        assert_eq!(client_secret, acc.get_client_secret());
        assert_eq!(refresh_token, acc.get_refresh_token());
    }
}
