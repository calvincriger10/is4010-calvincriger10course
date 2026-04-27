#[derive(Debug, PartialEq, Clone)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

pub fn validate_strength(password: &str) -> PasswordStrength {
    let len = password.len();
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_symbol = password.chars().any(|c| !c.is_alphanumeric());

    match (len, has_lower, has_upper, has_digit, has_symbol) {
        (l, _, _, _, true) if l >= 12 => PasswordStrength::VeryStrong,
        (l, true, true, true, _) if l >= 8 => PasswordStrength::Strong,
        (l, true, true, _, _) if l >= 8 => PasswordStrength::Medium,
        _ => PasswordStrength::Weak,
    }
}

pub fn check_common_patterns(password: &str) -> bool {
    let lower = password.to_lowercase();

    lower.contains("123")
        || lower.contains("456")
        || lower.contains("789")
        || lower.contains("qwerty")
        || lower.contains("asdf")
        || lower
            .as_bytes()
            .windows(3)
            .any(|w| w[0] == w[1] && w[1] == w[2])
}

pub fn calculate_entropy(password: &str) -> f64 {
    let mut charset_size: f64 = 0.0;

    if password.chars().any(|c| c.is_lowercase()) {
        charset_size += 26.0;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        charset_size += 26.0;
    }
    if password.chars().any(|c| c.is_numeric()) {
        charset_size += 10.0;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        charset_size += 8.0;
    }

    if charset_size == 0.0 {
        0.0
    } else {
        password.len() as f64 * charset_size.log2()
    }
}
