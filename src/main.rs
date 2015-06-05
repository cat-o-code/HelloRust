use std::io;

fn readoneline(){
	let mut reader = io::stdin();
	let mut input_text = String::new();
	reader.read_line(&mut input_text).ok().expect("failed to read line");
	let input_opt: Option<i32> = input_text.trim().parse::<i32>().ok();
	let input_int = match input_opt {
			Some(input_int) => {
				input_int;
			},
			None			=> {
				println!("please input a number");
				return;
			}
	};	
	return;
}

fn main() {



	println!("Please input 1st number");
	
	let mut number = readoneline();

	println!("{:?}", number);
}