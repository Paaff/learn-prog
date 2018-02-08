/*  CLI tool to download the contents of webpages.
 *  Implemented as an exercise in Rust with a state machine design in mind.
 */

// Crates
use std::env;
use std::io;

// Different states.
enum State {
    Start,
    Idle,
    Downloading,

}

// Welcome information printed to the user.
fn welcome() {
    println!("Welcome to the CLI tool for downloading webpages.");
}

// Listen for user input.
fn listen() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read the user input.");

    println!("You gave me this input: {}", input);
}

fn main() {
    // Initilize tool & check for arguments.
    welcome();

    for argument in env::args() {
        match argument.as_ref() {
            "--help"    => println!("Placeholder helping section."),
            "-h"        => println!("Placeholder helping section. (Shorthand tag used)"),
            _           => println!("No arguments were provided. Please proceed to input a command.")
        }
    }

    // Start of the state machine.


}
