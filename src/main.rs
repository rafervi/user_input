use std::io;
/// Crate comment.
/// What is this module trying to achieve.
fn main() {

    //! # Main function
    //! ````
    //! fn main ()
    //! ````
    //! Reads user input and prints it to the console.
    println!("Hello World!");
    println!("My name is {} and I'm {} years old", "Alex", 29);
    println!("a + b = {}",3 + 9);
    println!("{0} has a {2} and {0} has a {1}", "Alex", "cat", "dog");
    println!("{name} {surname} ", surname = "Smith", name="Alex");
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 50, 50, 50);
    println!("array: {:?}", [1, 2, 3]);

    let mut input =String::new();

    //print a message to the user
    println!("Say Something");
    /*
    Check response and act accordingly
     */
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);

        },
        Err(e) => {
            println!("Something went wrong{}",e);
        }
    }
}
