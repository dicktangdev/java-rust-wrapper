use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command};
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[macro_use]
extern crate litcrypt;
use_litcrypt!();

fn start_listener(app_path: String, sender: Sender<String>) {
    let java_app_path = app_path;
    let java_agent_argument = format!("-{}:{}={}", lc!("javaagent"), java_app_path, lc!("-pwd Alfred24"));
    
    let child = Command::new("java")
        .arg(java_agent_argument)
        .arg("-jar")
        .arg(java_app_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("cannot spawn");

    println!("Started process: {}", child.id());

    thread::spawn(move || {
        let mut f = BufReader::new(child.stdout.unwrap());
        loop {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    sender.send(buf).unwrap();
                }
                Err(e) => println!("an error!: {:?}", e),
            }
        }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided (excluding the program name)
    if args.len() > 1 {
        // The first argument (args[1]) is the one we're interested in
        let first_arg = &args[1];
        println!("[{} - {}]", lc!("App start by Rust"), first_arg);

        let (tx, rx) = channel();
        start_listener(first_arg.to_string(), tx);
    
        for line in rx {
            print!("{}", line);
        }
    
        println!("{}", lc!("Done!"));
    } else {
        println!("{}", lc!("Error: No argument provided."));
        return;
    }

   
}