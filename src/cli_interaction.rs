use dialoguer::Confirm;
use dialoguer::Input;
use dialoguer::Password;

pub fn get_credentials() -> (String, String) {
    let username: String = Input::new()
        .with_prompt("Username")
        .default("tf-test".into())
        .allow_empty(false)
        .interact()
        .unwrap_or_default();

    let password = Password::new()
        .with_prompt("Password")
        .interact()
        .unwrap_or_default();

    (username, password)
}

pub fn retry_credentials() -> bool {
    Confirm::new()
        .with_prompt("Unknown username/password. Retry")
        .default(true)
        .interact()
        .unwrap_or_default()
}
