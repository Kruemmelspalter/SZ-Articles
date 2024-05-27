use regex_macro::regex;

pub fn is_valid_email(email: &str) -> bool {
    regex!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").is_match(email)
}

pub fn is_valid_username(username: &str) -> bool {
    regex!(r"^[a-zA-Z0-9._%+-]{6,}$").is_match(username)
}

pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 6
        && password.chars().any(|c| c.is_alphabetic())
        && password.chars().any(|c| c.is_numeric())
        && password.chars().any(|c| !c.is_alphanumeric())
}
