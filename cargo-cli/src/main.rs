use std::io::Write;
use std::io;
use std::process::{Command, Output};

fn main() {
    	println!("Hello and welcome to RITA!");
	
	let mut command = String::new();

	while command.trim().to_lowercase() != "exit" {
		print!("||[: ");
		std::io::stdout().flush().unwrap();
		io::stdin()
			.read_line(&mut command)
			.expect("Failed to read line");
		// print!("{command}");
		if command.trim().to_lowercase() == "exit" {
			break;
		
		}
		if command.trim().to_lowercase() == "pwd" {
			let output: Output = Command::new("pwd")
			    .output()
			    .expect("Failed to run command");
			if output.status.success() {
				let pwd_output = String::from_utf8_lossy(&output.stdout);
				
				print!("{}", pwd_output);
    			} else {
        			eprintln!("Error running command: {}", String::from_utf8_lossy(&output.stderr));
    			}
		}
        if command.trim().to_lowercase() == "ls" {
            let output: Output = Command::new("ls")
                .output()
                .expect("Failed to run command");
            if output.status.success() {
                let ls_output = String::from_utf8_lossy(&output.stdout);
                
                print!("{}", ls_output);
                } else {
                    eprintln!("Error running command: {}", String::from_utf8_lossy(&output.stderr));
                }
        }
        if command.trim().to_lowercase() == "rita?" {
            print!("RITA Stands for Rust Integrated Terminal Application\n");
        }
        if command.trim().to_lowercase() == "help" {
            print!("Current Command List
            -| ls   - List Files/Directories in Current Folder.
            -| pwd  - Show Current File Path.
            -| exit - Exit RITA.\n");
        }
		command = String::new();
	}
}
