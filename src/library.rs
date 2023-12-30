use std::{fs::{OpenOptions, File}, io::{BufReader, BufRead, Write}};

pub fn help() {
	println!("Func: Help!");
}

pub fn list() {
	let file = open_csv("data/tasks.csv");
	let reader = BufReader::new(file);

	let lines:Vec<String> = reader
		.lines()
		.filter_map(|line| line.ok())
		.collect();

	for data in lines {
		println!("{}", data);
	}

	println!("Func: List!");
}

pub fn edit(_options: Vec<String>) {
	println!("Func: Edit!");
}

pub fn add(arguments: Vec<String>) {
	if arguments.len() < 1 {
		eprintln!("Missing arguments need a task !");
		std::process::exit(127);
	}

	let mut file: File = open_csv("data/tasks.csv");
	
	let mut task:String = String::new();

	for elem in arguments {
		task.push_str(elem.as_str());
		task.push_str(" ");
	}

	task = task[0..task.len() - 1].to_string();
	task.push_str(",nd\n");

	match file.write_all(task.as_bytes()) {
		Ok(_) => println!("Task added to your list"),
		Err(err) => {
			eprintln!("{}: couldn't add your task", err);
			std::process::exit(55);
		}
	}
}


fn open_csv(file_name: &str) -> File {
	
	let file = OpenOptions::new()
		.read(true)
		.write(true)
		.append(true)
		.create(true)
		.open(file_name);
	
	match file {
		Ok(file) => return file,
		Err(err) => {
			eprintln!("{}: \"{}\"", err, file_name);
			std::process::exit(127);
		}
	}
}