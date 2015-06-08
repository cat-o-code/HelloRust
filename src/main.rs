use std::io;
use std::thread::sleep_ms;

fn pressanykey() {
	sleep_ms(3000);
} // will get to this later, poor windows

fn readnumber(message: &'static str) -> (f64,bool){
	let mut reader = io::stdin();
	println!("{}", message);
	loop {
		let mut input_text = String::new();
		reader.read_line(&mut input_text).ok();
		let input_opt: Option<f64> = input_text.trim().parse::<f64>().ok();
		let input_int = match input_opt {
			Some(input_int) => {
				return (input_int,true);
			},
			None			=> {
				println!("{}, again", message);
				continue;
			}
		};
	}	
	return (0.0,false);
}

fn main() {
	let mut number1 = readnumber("Please input 1st number");
	let mut number2 = readnumber("Please input 2nd number");

	println!("SUM is {0:?}", number1.0 + number2.0);
	println!("DIF is {0:?}", number1.0 - number2.0);
	println!("PRO is {0:?}", number1.0 * number2.0);
	if number2.0 != 0.0 {
		println!("DIV is {0:?}", number1.0 / number2.0);
	} else {
		println!("DIV is {0:?}", "INFINITY");
	}

	pressanykey();
}