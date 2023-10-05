// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let suffix : &str = "Hello world";
    println!("Hello {}!" ,suffix.to_string());
}
