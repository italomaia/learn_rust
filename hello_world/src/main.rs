// this is the starting point of your rust program
fn main() {  // it can receive modifiers like `async` or return a Result
    // println! is a very useful macro; it prints to stdout
    println!("Hello, world!");
    // you can print primitives
    println!("Hello, {} Doe!", "John");
    // you can style output
    println!("I'm {:02} years old.", 8);
    // my age in binary is...
    println!("This much: {:b}!", 8);
    // you can easely print data structures that implement Debug
    // println!("This is a tuple: {}", (42, 84));  // this fails
    println!("This is a tuple: {:?}", (42, 84));   // this works
}
