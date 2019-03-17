use std::env; // Allow env variables
use std::error::Error; // Allow errors
use std::fs; // Allow filesystem
use std::path::PathBuf; // Allow path buffers
use std::process::Command; // Allow cmd

extern crate dirs; // Import dirs package

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
        "run" => handle_run(),             // Run
        "get" => handle_get(args),         // Get
        "install" => handle_install(args), // Install
        "build" => handle_build(),         // Build
        "init" => handle_init(args),       // Init
        _ => println!("invalid action"),   // Default
    } // Handle different actions
}

// handle_run handles the run command.
fn handle_run() {
    let output = Command::new("cargo").arg("run").output().expect(""); // Build cargo

    println!("{}", String::from_utf8_lossy(&output.stdout)); // Log build
}

// handle_get handles the get command.
fn handle_get(args: Vec<String>) {
    let home_dir: Option<PathBuf> = dirs::home_dir(); // Get home dir

    let mut home_dir_str: String; // Init home dir buffer

    let path_split: Vec<&str> = args[2].split("/").collect(); // Split path

    match home_dir {
        Some(x) => {
            let home_dir_str_temp = x.into_os_string().into_string();
            match home_dir_str_temp {
                Ok(v) => home_dir_str = v,
                _ => {
                    println!("failed"); // Log fail

                    return; // Stop execution
                }
            }
        } // Set home dir str val
        None => {
            println!("could not get home directory"); // Log failure

            return; // Stop execution
        } // Stop execution
    } // Get option value

    let result = fs::create_dir_all(
        &[
            home_dir_str.to_string(),
            String::from("/rust/src/"),
            String::from(path_split[0]),
            String::from("/"),
            String::from(path_split[1]),
        ]
        .concat(),
    ); // Make rust dir

    match result {
        Ok(_v) => {}
        Err(e) => {
            println!("{}", e.description());
            return;
        } // Handle err
    }

    let output = Command::new("git")
        .arg("clone")
        .arg(
            &[
                String::from("https://"),
                args[2].to_string(),
                String::from(".git"),
            ]
            .concat(),
        )
        .current_dir(
            &[
                home_dir_str,
                String::from("/rust/src/"),
                String::from(path_split[0]),
                String::from("/"),
                String::from(path_split[1]),
            ]
            .concat(),
        )
        .output()
        .expect(""); // Get

    println!("{}", String::from_utf8_lossy(&output.stdout)); // Log output
}

// handle_install handles the install command.
fn handle_install(args: Vec<String>) {
    handle_get(args.to_owned()); // Get package

    let home_dir: Option<PathBuf> = dirs::home_dir(); // Get home dir

    let mut home_dir_str: String; // Init home dir buffer

    let path_split: Vec<&str> = args[2].split("/").collect(); // Split path

    match home_dir {
        Some(x) => {
            let home_dir_str_temp = x.into_os_string().into_string();
            match home_dir_str_temp {
                Ok(v) => home_dir_str = v,
                _ => {
                    println!("failed"); // Log fail

                    return; // Stop execution
                }
            }
        } // Set home dir str val
        None => {
            println!("could not get home directory"); // Log failure

            return; // Stop execution
        } // Stop execution
    } // Get option value

    let output = Command::new("cargo")
        .arg("install")
        .arg("--path")
        .arg(".")
        .current_dir(
            &[
                home_dir_str,
                String::from("/rust/src/"),
                String::from(path_split[0]),
                String::from("/"),
                String::from(path_split[1]),
            ]
            .concat(),
        )
        .output()
        .expect(""); // install

    println!("{}", String::from_utf8_lossy(&output.stdout)); // Log output
}

// handle_build handles the build command.
fn handle_build() {
    let output = Command::new("cargo").arg("build").output().expect(""); // install

    println!("{}", String::from_utf8_lossy(&output.stdout)); // Log output
}

// handle_init handles the init command.
fn handle_init(args: Vec<String>) {
    let home_dir: Option<PathBuf> = dirs::home_dir(); // Get home dir

    let mut home_dir_str: String; // Init home dir buffer

    let path_split: Vec<&str> = args[2].split("/").collect(); // Split path

    match home_dir {
        Some(x) => {
            let home_dir_str_temp = x.into_os_string().into_string();
            match home_dir_str_temp {
                Ok(v) => home_dir_str = v,
                _ => {
                    println!("failed"); // Log fail

                    return; // Stop execution
                }
            }
        } // Set home dir str val
        None => {
            println!("could not get home directory"); // Log failure

            return; // Stop execution
        } // Stop execution
    } // Get option value

    let result = fs::create_dir_all(
        &[
            home_dir_str.to_string(),
            String::from("/rust/src/"),
            String::from(path_split[0]),
            String::from("/"),
            String::from(path_split[1]),
        ]
        .concat(),
    ); // Make rust dir

    match result {
        Ok(_v) => {}
        Err(e) => {
            println!("{}", e.description());
            return;
        } // Handle err
    }

    let output = Command::new("cargo")
        .arg("init")
        .current_dir(
            &[
                home_dir_str,
                String::from("/rust/src/"),
                String::from(path_split[0]),
                String::from("/"),
                String::from(path_split[1]),
                String::from(path_split[2]),
            ]
            .concat(),
        )
        .output()
        .expect(""); // init

    println!("{}", String::from_utf8_lossy(&output.stdout)); // Log output
}
