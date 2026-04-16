fn main() {
    println!("Lab 10: Mastering Ownership and Borrowing");
    println!("All problems fixed!\n");

    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();

    let owned = to_uppercase_owned(String::from("rust"));
    println!("Uppercase: {}", owned);

    let s = String::from("hello");
    println!("Length: {}", string_length(&s));

    let mut s2 = String::from("hello");
    append_suffix(&mut s2, " there");
    println!("Appended: {}", s2);

    let combined = concat_strings("Rust", "Lang");
    println!("Combined: {}", combined);

    let word = first_word("hello world");
    println!("First word: {}", word);
}

// ================= PROBLEMS =================

fn problem_1() {
    println!("Problem 1");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &str) -> (&str, usize) {
    (s, s.len())
}

fn problem_2() {
    println!("Problem 2");
    let mut s = String::from("hello");
    let r1 = &s;
    println!("  {}", r1);
    let r2 = &mut s;
    println!("  {}", r2);
}

fn problem_3() {
    println!("Problem 3");
    let mut s = String::from("hello");
    add_to_string(&mut s);
    println!("  {}", s);
}

fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

fn problem_4() {
    println!("Problem 4");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("  {}", r1);
    }

    let r2 = &mut s;
    println!("  {}", r2);
}

fn problem_5() {
    println!("Problem 5");
    let r = create_string();
    println!("  {}", r);
}

fn create_string() -> String {
    String::from("hello")
}

fn problem_6() {
    println!("Problem 6");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i);
    }
}

fn print_with_number(s: &str, n: i32) {
    println!("  {}: {}", n, s);
}

fn problem_7() {
    println!("Problem 7");
    let s = String::from("inner scope");
    let result = &s;
    println!("  {}", result);
}

// ================= IMPLEMENTATION =================

fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

fn string_length(s: &str) -> usize {
    s.len()
}

fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// ================= TESTS =================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length_borrows() {
        let s = String::from("testing");
        let (_s_ref, len) = calculate_length(&s);
        assert_eq!(len, 7);
        assert_eq!(s, "testing");
    }

    #[test]
    fn test_add_to_string_mutates() {
        let mut s = String::from("hello");
        add_to_string(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_create_string_returns_owned() {
        let result = create_string();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_print_with_number_borrows() {
        let data = String::from("Rust");
        for i in 0..3 {
            print_with_number(&data, i);
        }
        assert_eq!(data, "Rust");
    }
}
