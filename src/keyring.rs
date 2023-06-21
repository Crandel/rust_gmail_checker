use keyring::{Entry, Result};

const SERVICE_NAME: &str = "gmail_checker";

pub fn get_password(key: String) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, key.as_str())?;
    let password = entry.get_password()?;

    return Ok(password);
}

pub fn set_password(key: String, password: String) -> bool {
    let entry = Entry::new(SERVICE_NAME, key.as_str())?;
    let password = entry.set_password(password.as_str())?;

    return true;
}
