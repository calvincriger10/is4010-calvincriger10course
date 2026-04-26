// Lab 10: The Borrow Checker Game

fn main() {
    println!("Lab 10: Mastering Ownership and Borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

// ============================================================================
// PROBLEM 1: Fixed - calculate_length now borrows instead of taking ownership
// ============================================================================
fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// ============================================================================
// PROBLEM 2: Fixed - use r1 before creating mutable borrow
// ============================================================================
fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s;
    println!("  {}", r1); // r1 last used here, scope ends
    let r2 = &mut s; // now OK
    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 3: Fixed - use &mut String and mut variable
// ============================================================================
fn problem_3() {
    println!("Problem 3: Mutating through immutable reference");
    let mut s = String::from("hello");
    add_to_string(&mut s);
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Fixed - use scopes to separate mutable borrows
// ============================================================================
fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("  {}", r1);
    } // r1 goes out of scope here
    let r2 = &mut s; // now OK
    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 5: Fixed - return owned String instead of dangling reference
// ============================================================================
fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    String::from("hello")
}

// ============================================================================
// PROBLEM 6: Fixed - borrow data in loop instead of moving
// ============================================================================
fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");
    for i in 0..3 {
        print_with_number(&data, i);
    }
}

fn print_with_number(s: &str, n: i32) {
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Fixed - move String declaration outside the inner scope
// ============================================================================
fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let s = String::from("inner scope");
    let result = &s;
    println!("  Result: {}", result);
}

// ============================================================================
// IMPLEMENTATION EXERCISES
// ============================================================================

/// Takes ownership of a String, converts it to uppercase, and returns it.
#[allow(dead_code)]
fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

/// Borrows a string slice and returns its length.
#[allow(dead_code)]
fn string_length(s: &str) -> usize {
    s.len()
}

/// Borrows a String mutably and appends a suffix to it.
#[allow(dead_code)]
fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

/// Creates a new String by concatenating two borrowed strings.
#[allow(dead_code)]
fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

/// Finds the first word in a string and returns it as a string slice.
#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// ============================================================================
// TEST SUITE
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length_borrows() {
        let s = String::from("testing");
        let len = calculate_length(&s);
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

    #[test]
    fn test_to_uppercase_owned() {
        let s = String::from("hello");
        let result = to_uppercase_owned(s);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_string_length() {
        let s = String::from("hello");
        assert_eq!(string_length(&s), 5);
    }

    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, " world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_strings("hello", " world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
    }
}