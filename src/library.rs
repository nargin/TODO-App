use std::{fs::{OpenOptions, File, }, io::{BufReader, BufRead, Write}};
use std::fs;

pub fn list() {
	let file = open_csv("data/tasks.csv");
	let reader = BufReader::new(file);
	
	let mut lines:Vec<String> = reader
		.lines()
		.map(|line| line.unwrap())
		.collect();

	for line in &mut lines {
		*line = line.trim().to_string();
	}

	let mut index:u32 = 1;

	for content in lines.iter() {
		if content.ends_with(".d") {
			let trun = &content[0..content.len() - 2];
			println!("{}. \x1B[9m{}\x1B[0m", index, trun);
		} else if content.len() > 1 {
			println!("{}. {}", index, content);
		} else {
			continue;
		}
		index += 1;
	}
}

pub fn add(arguments: Vec<String>) {
	if arguments.len() < 1 {
		eprintln!("Missing arguments need a task !");
		std::process::exit(127);
	}
	
	let mut file: File = open_csv("data/tasks.csv");
	
	for content in &arguments {
		let content = content.to_owned() + "\n";
		match file.write_all(content.as_bytes()) {
			Ok(_) => continue,
			Err(err) => {
				eprintln!("{}: couldn't add your task", err);
				std::process::exit(55);
			}
		}
	}
	println!("{} task/s added to the list !", arguments.len());
}

pub fn edit(arguments: Vec<String>) {
	if arguments.len() != 2 && arguments[1].len() >= 2 {
		eprintln!("Arguments not matching: Try ./todo help");
		std::process::exit(127);
	}
	
	let nrow:usize = match arguments[0].parse() {
		Ok(number) => number,
		Err(err) => {
			eprintln!("Number input error: {}", err);
			std::process::exit(1);
		}
	};

	let mut file = open_csv("data/tasks.csv");
	let reader = BufReader::new(file);
	
	let mut lines:Vec<String> = reader
		.lines()
		.map(|line| line.unwrap())
		.collect();

	if nrow <= lines.len() {
		let nrow = nrow - 1;
		
		if lines[nrow].ends_with(".d") {
			lines[nrow] = arguments[1].to_string() + ".d";
		} else {
			lines[nrow] = arguments[1].to_string();
		}

		file = fs::File::create("./data/tasks.csv").expect("Unable to write");
		for line in &lines {
			writeln!(&mut file, "{}", line).expect("Unable to write");
		}
	}
	println!("Line {} edited", nrow);
}

pub fn task_done(arguments: Vec<String>) {
	
	if arguments.len() < 1 {
		std::process::exit(1);
	}

	let mut row:Vec<usize> = Vec::new();

	for arg in arguments {
		let nrow:usize = match arg.parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		row.push(nrow);
	}

	row.sort();

	let mut file = open_csv("data/tasks.csv");
	let reader = BufReader::new(file);
	
	let mut lines:Vec<String> = reader
		.lines()
		.map(|line| line.unwrap())
		.collect();

	for num in row {
		if num <= lines.len() && lines[num - 1].ends_with(".d") == false {
			let num = num - 1;
			lines[num] = lines[num].to_string() + ".d";
			println!("Task {} is now done", num + 1);
		}
	}

	file = fs::File::create("./data/tasks.csv").expect("Unable to write");
	for line in &lines {
		writeln!(&mut file, "{}", line).expect("Unable to write");
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

pub fn delete() {
	if fs::metadata("./data/tasks.csv").is_ok() {
		let _ = match fs::remove_file("./data/tasks.csv") {
			Ok(_) => File::create("./data/tasks.csv"),
			Err(err) => {
				eprintln!("Error when deleting: \"{}\"", err);
				std::process::exit(127);
			}
		};
	}
}

const HELP_MESSAGE: &str = "Usage: ./todo [CMD] [ARGS]
TODO is a simple tasks list written in Rust
Use case : ./todo add buy apples
All commands: 
	- list
		lists all tasks
		./todo list
	- add [task/s]
		add a task to the list
		./todo add \"buy coconuts\"
	- done [index/s]
		mark a task as done
		./todo done 2
	- edit [index] [task]
		edit the content of a task
		./todo edit 2 \"new content\"
	- rm [index]
		obliterate the task
		./todo rm 1
	- del
		reset everything, factory reset
		./todo del
	- help
		this
		./todo help
";

pub fn help() {
	println!("{}", HELP_MESSAGE);
}