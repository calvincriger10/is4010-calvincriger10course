use rand::Rng;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*";

pub fn generate_random(length: usize, use_symbols: bool) -> String {
    let mut rng = rand::thread_rng();

    let mut charset = String::new();
    charset.push_str(LOWERCASE);
    charset.push_str(UPPERCASE);
    charset.push_str(DIGITS);

    if use_symbols {
        charset.push_str(SYMBOLS);
    }

    let chars: Vec<char> = charset.chars().collect();

    (0..length)
        .map(|_| {
            let index = rng.gen_range(0..chars.len());
            chars[index]
        })
        .collect()
}

pub fn generate_pin(length: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..length)
        .map(|_| rng.gen_range(0..10).to_string())
        .collect()
}

pub fn generate_passphrase(word_count: usize, separator: char) -> String {
    let words = [
        "correct", "horse", "battery", "staple", "rust", "secure", "apple", "river",
    ];

    let mut rng = rand::thread_rng();

    (0..word_count)
        .map(|_| words[rng.gen_range(0..words.len())])
        .collect::<Vec<&str>>()
        .join(&separator.to_string())
}
