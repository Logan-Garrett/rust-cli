use std::io::Write;
use std::io;

fn main() {
    	println!("Hello and welcome to 'name'!");
	
	let mut command = String::new();

	while command.trim().to_lowercase() != "exit" {
		print!("||[: ");
		std::io::stdout().flush().unwrap();
		io::stdin()
			.read_line(&mut command)
			.expect("Failed to read line");
		println!("{command}");
		if command.trim().to_lowercase() == "exit" {
			break;
		}
		command = String::new();
	}
}
