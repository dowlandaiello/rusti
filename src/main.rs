use std::env; // Allow env variables
use std::process::Command; // Allow cmd

// main is the entry-point for the rust utility.
fn main() {
    let args: Vec<String> = env::args().collect(); // Get CLI args

    if args.len() == 1 {
        // Check no args
        println!("invalid input (no params)"); // Log failure

        return; // Done
    }

    let action = &args[1]; // Get action

    match action.as_ref() {
        "run" => handle_run(),           // Run
        _ => println!("invalid action"), // Default
    } // Handle different actions
}

// handle_run handles the run command.
fn handle_run() {
    let output = Command::new("cargo").arg("run").output().expect(""); // Build cargo

    println!("{}", String::from_utf8_lossy(&output.stdout)); // Log build
}
