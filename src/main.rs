mod library;

use std::{env, fs};
use library::{add, help, list, edit, delete, task_done};

fn main()
{	
	if fs::metadata("./data/").is_ok() == false {
		match fs::create_dir("./data/") {
			Ok(_) => println!("data directory created"),
			Err(err) => {
				eprintln!("Error uppon creation folder: {}", err);
				std::process::exit(127);
			}
		}
	}

	let mut args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		list();
		std::process::exit(0);
	}
	
	let cmd = {
		let arg1 = args[1].clone();

		match args.get(2) {
			Some(_) => args = args[2..].to_vec(),
			None => args.clear(),
		}
		arg1
	};

	match &cmd[..] {
		"list" => list(),
		"add" => add(args),
		"done" => task_done(args),
		"edit" => edit(args),
		"del" => delete(),
		"help" => help(),
		_ => {
			eprintln!("Command not found, try ./todo help !");
			std::process::exit(127)
		},
	}

	// println!("Command is \"{cmd}\"");
	std::process::exit(0);
}