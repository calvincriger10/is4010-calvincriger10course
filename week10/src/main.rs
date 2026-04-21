// Lab 10: The Borrow Checker Game
// Fix each problem one at a time by uncommenting the function call in main()

fn main() {
    println!("Lab 10: Mastering Ownership and Borrowing");
    println!("Uncomment one problem at a time and fix it!\n");

    // Uncomment problems one at a time:
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

// ============================================================================
// PROBLEM 1: Value used after move
// ============================================================================

fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(&s1); // add & here
    println!("  The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &String) -> (String, usize) {
    let length = s.len();
    (s.clone(), length)
}

// ============================================================================
// PROBLEM 2: Immutable and mutable borrow conflict
// ============================================================================

fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s;
    println!("  {}", r1); // r1 used and done here
    let r2 = &mut s; // now this is fine!
    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 3: Mutating through immutable reference
// ============================================================================

fn problem_3() {
    println!("Problem 3: Mutating through immutable reference");
    let mut s = String::from("hello"); // add mut here
    add_to_string(&mut s); // change &s to &mut s
    println!("  Result: {}", s);
}
fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

// ============================================================================
// PROBLEM 4: Multiple mutable borrows
// ============================================================================

fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("  {}", r1);
    } // r1 goes out of scope here, mutable borrow ends

    let r2 = &mut s; // now this is fine!
    println!("  {}", r2);
}

// ============================================================================
// PROBLEM 5: Dangling reference
// ============================================================================

fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    let s = String::from("hello");
    s // give ownership to whoever called this function
}

// ============================================================================
// PROBLEM 6: Ownership in loops
// ============================================================================

fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");

    for i in 0..3 {
        print_with_number(&data, i); // borrow instead of move
    }
}

fn print_with_number(s: &str, n: i32) {
    println!("  {}: {}", n, s);
}

// ============================================================================
// PROBLEM 7: Lifetime extension challenge
// ============================================================================

fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let s = String::from("inner scope");
    let result = &s; // s lives long enough now!
    println!("  Result: {}", result);
}

// ============================================================================
// IMPLEMENTATION EXERCISES
// ============================================================================

fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

fn string_length(s: &String) -> usize {
    s.len()
}

fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix)
}

fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[0..i],
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
