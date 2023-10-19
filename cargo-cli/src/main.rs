use std::io::Write;
use std::io;

fn main() {
    	println!("Hello and welcome to 'name'!");

	print!("||[: ");
	std::io::stdout().flush().unwrap();
	let mut command = String::new();

	io::stdin()
		.read_line(&mut command)
		.expect("Failed to read line");

	println!("Command entered: {command}");
}
