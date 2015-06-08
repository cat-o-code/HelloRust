use std::io;



fn readoneline() -> (i64,bool){
	let mut reader = io::stdin();
	let mut input_text = String::new();

	reader.read_line(&mut input_text).ok().expect("failed to read line");
	let input_opt: Option<i64> = input_text.trim().parse::<i64>().ok();
	let input_int = match input_opt {
			Some(input_int) => {
				return (input_int,true);
			},
			None			=> {
				println!("The input was not a number");
				return (0,false);
			}
	};	
	return (0,false);
}

fn main() {

	let mut number1; 
	let mut number2;

	loop {	
		println!("Please input 1st number");
		number1 = readoneline();
		if number1.1 {
			break;
		} else {
			continue;
		}
	}
	loop {	
		println!("Please input 2st number");
		number2 = readoneline();
		if number2.1 {
			break;
		} else {
			continue;
		}
	}

	println!("SUM is {0:?}", number1.0 + number2.0);
	println!("DIF is {0:?}", number1.0 - number2.0);
	println!("PRO is {0:?}", number1.0 * number2.0);
	println!("DIV is {0:?}", number1.0 / number2.0);

}