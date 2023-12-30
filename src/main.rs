mod library;

use std::env;
use library::{add, help, list, edit};



fn main()
{
	let mut args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		list();
		std::process::exit(0);
	}

	let cmd = {
		let a1 = args[1].clone();

		match args.get(2) {
			Some(_) => args = args[2..].to_vec(),
			None => args.clear(),
		}

		a1
	};

	match &cmd[..] {
		"add" => add(args),
		"list" => list(),
		"help" => help(),
		"edit" => edit(args),
		_ => {
			eprintln!("Command not found, try ./todo help !");
			std::process::exit(127)
		},
	}

	// println!("Command is \"{cmd}\"");
	std::process::exit(0);
}