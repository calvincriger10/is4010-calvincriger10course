use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::rc::Rc;

fn analyze_text(text: &str) -> (usize, f64, String) {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return (0, 0.0, String::new());
    }

    let word_count = words.len();
    let total_length: usize = words.iter().map(|w| w.len()).sum();
    let avg = total_length as f64 / word_count as f64;

    let longest = words
        .iter()
        .fold("", |longest, word| {
            if word.len() > longest.len() {
                word
            } else {
                longest
            }
        })
        .to_string();

    (word_count, avg, longest)
}

fn process_numbers(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum()
}

fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

#[derive(Debug, PartialEq)]
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree::Empty
    }

    fn leaf(value: T) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }
    }

    fn node(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
struct SharedData {
    value: i32,
}

#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Rc<RefCell<Counter>> {
        Rc::new(RefCell::new(Counter { value: 0 }))
    }

    fn increment(c: &Rc<RefCell<Counter>>) {
        c.borrow_mut().value += 1;
    }

    fn get_value(c: &Rc<RefCell<Counter>>) -> i32 {
        c.borrow().value
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn read_file_contents(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug, Clone, PartialEq)]
enum ParseError {
    EmptyInput,
    InvalidNumber(String),
    OutOfRange(i32),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty input"),
            ParseError::InvalidNumber(s) => write!(f, "Invalid: {}", s),
            ParseError::OutOfRange(n) => write!(f, "Out of range: {}", n),
        }
    }
}

fn parse_positive_number(input: &str) -> Result<i32, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let num: i32 = input
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidNumber(input.to_string()))?;

    if !(1..=100).contains(&num) {
        return Err(ParseError::OutOfRange(num));
    }

    Ok(num)
}

#[derive(Debug, Clone)]
struct Config {
    min_length: usize,
    max_length: usize,
}

#[derive(Debug, PartialEq)]
enum ProcessError {
    LineTooShort(String),
    LineTooLong(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessError::LineTooShort(line) => write!(f, "Line too short: {}", line),
            ProcessError::LineTooLong(line) => write!(f, "Line too long: {}", line),
        }
    }
}

fn process_lines(
    lines: &[String],
    config: Rc<RefCell<Config>>,
) -> Result<Vec<String>, ProcessError> {
    lines
        .iter()
        .map(|line| {
            let cfg = config.borrow();

            if line.len() < cfg.min_length {
                Err(ProcessError::LineTooShort(line.clone()))
            } else if line.len() > cfg.max_length {
                Err(ProcessError::LineTooLong(line.clone()))
            } else {
                Ok(line.to_uppercase())
            }
        })
        .collect()
}

fn main() {
    println!("Lab 13 Running");

    let stats = analyze_text("hello world rust");
    println!("{:?}", stats);

    println!("{}", process_numbers(&[1, 2, 3, 4]));

    let mut counter = make_counter();
    println!("{}", counter());

    let _empty: BinaryTree<i32> = BinaryTree::new();
    let _tree = BinaryTree::node(5, BinaryTree::leaf(3), BinaryTree::leaf(7));

    let shared = Rc::new(SharedData { value: 42 });
    let shared_clone = Rc::clone(&shared);
    println!("{}", shared_clone.value);

    let c = Counter::new();
    Counter::increment(&c);
    println!("{}", Counter::get_value(&c));

    println!("{:?}", divide(10.0, 2.0));
    println!("{:?}", read_file_contents("missing_file.txt"));
    println!("{:?}", parse_positive_number("50"));

    let config = Rc::new(RefCell::new(Config {
        min_length: 3,
        max_length: 10,
    }));
    let lines = vec!["hello".to_string()];
    println!("{:?}", process_lines(&lines, config));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_text() {
        let r = analyze_text("hello world");
        assert_eq!(r.0, 2);
        assert_eq!(r.2, "hello");
    }

    #[test]
    fn test_numbers() {
        assert_eq!(process_numbers(&[1, 2, 3, 4]), 20);
    }

    #[test]
    fn test_counter() {
        let mut c = make_counter();
        assert_eq!(c(), 1);
        assert_eq!(c(), 2);
    }

    #[test]
    fn test_tree() {
        let t = BinaryTree::node(5, BinaryTree::leaf(3), BinaryTree::leaf(7));
        match t {
            BinaryTree::Node { value, .. } => assert_eq!(value, 5),
            _ => panic!(),
        }
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_positive_number("50"), Ok(50));
        assert!(parse_positive_number("").is_err());
    }

    #[test]
    fn test_process_lines() {
        let config = Rc::new(RefCell::new(Config {
            min_length: 3,
            max_length: 10,
        }));

        let lines = vec!["hello".to_string()];
        let result = process_lines(&lines, config);

        assert_eq!(result.unwrap(), vec!["HELLO"]);
    }
}
