use keyring::{Entry, Result};

const SERVICE_NAME: &str = "gmail_checker";

pub fn get_entry(key: String) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, key.as_str())?;
    let password = entry.get_password();

    return password;
}

pub fn set_entry(key: String, data: String) -> bool {
    let resultEntry = Entry::new(SERVICE_NAME, key.as_str());
    match resultEntry {
        Ok(entry) => {
            if let Ok(password) = entry.set_password(data.as_str()) {
                return true;
            };
        }
        Err(err) => println!("{:?}", err),
    };

    return false;
}
