use crate::commands::AuthArgs;
use crate::hash::hash;
use rpassword;

const SEPERATOR: &str = "|";

pub fn build_credentials(auth_args: &AuthArgs) {
    // TODO slim down to one vector until final push
    let mut tokens: Vec<String> = Vec::new();
    let mut redacted_tokens: Vec<String> = Vec::new();

    if let Some(domain) = &auth_args.domain {
        println!("Generating password with domain: {}...", domain); // TODO: verbose 1
        tokens.push(domain.clone());
        redacted_tokens.push(domain.clone());
    }

    if let Some(user) = &auth_args.user {
        println!("Generating password with user: {}...", user); // TODO: verbose 1
        tokens.push(user.clone());
        redacted_tokens.push(user.clone());
    }

    let password = request_master_key();
    println!("Generating password with master key"); // TODO: verbose 1
    tokens.push(password);
    redacted_tokens.push(String::from("[REDACTED]"));

    let key = tokens.join(SEPERATOR);
    let redacted_key = redacted_tokens.join(SEPERATOR);

    println!("Composite Key: {redacted_key}"); // TODO: verbose 2 or 3

    let hashed = hash(key);

    println!("{hashed}");
}

// TODO would be neat if we could just grab this from flag, stdin, or ENV:
// trashword auth -d example.com -p password
// cat master.key | trashword auth example.com
// TRASHWORD_MASTER='foo' trashword auth example.com
fn request_master_key() -> String {
    match rpassword::prompt_password("master key: ") {
        Ok(password) => password,
        Err(error) => {
            panic!("A problem occured reading master key: {}", error)
        }
    }
}
