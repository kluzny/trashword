use rpassword;
use crate::commands::AuthArgs;

pub fn build_credentials(auth_args: &AuthArgs) {
    let domain = extract_domain(auth_args);
    let password = request_master_key();

    let key = format!("{domain}|{password}");
    println!("{key}");
}

fn extract_domain(auth_args: &AuthArgs) -> String {
    // TODO hide behind verbose
    if auth_args.domain.len() > 0 {
        println!("Generating password for {}...", auth_args.domain);
    } else {
        println!("Generating password ...");
    }

    auth_args.domain.clone()
}

// TODO would be neat if we could just grab this from stdin or ENV:
// cat master.key | trashword auth example.com
// TRASHWORD_MASTER='foo' trashword auth example.com
fn request_master_key() -> String {
    match rpassword::prompt_password("master key: ") {
        Ok(password) => { password },
        Err(error) => { panic!("A problem occured reading master key: {}", error) }
    }
}