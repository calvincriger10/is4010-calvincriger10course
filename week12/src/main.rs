fn main() {
    println!("Lab 12: Generic Stack Implementation\n");

    let mut int_stack = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    int_stack.push(30);

    println!("Stack: {}", int_stack);

    // 👇 ADD THESE TWO LINES
    println!("Length: {}", int_stack.len());
    println!("Is empty? {}", int_stack.is_empty());

    println!("Popped: {:?}", int_stack.pop());
    println!("Peek: {:?}", int_stack.peek());

    println!("\nIterating:");
    for item in int_stack {
        println!("{}", item);
    }
}
