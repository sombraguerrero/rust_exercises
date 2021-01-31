use std::io::{self, Write};
//This program uses the input-stream crate (v0.4.0 as of 1/30/21) as a flexible workaround for the limitation of Windows console apps not implementing buffer reads on non-UTF8 data
use input_stream::InputStream;
fn main() -> io::Result<()> {
	let mut decision: i32 = 0;
	while decision != 6 {
		decision = menu();
		if decision == 1 {
			obt();
		}
		else if decision == 2 {
			print!("Specify Farenheit temperature to convert: ");
			let _ = io::stdout().flush(); // necessary because Rust does not flush output streams without newlines appended. (as of v1.51, 1/30/21)
			println!("Converted temperature is {} degrees Celsius!", convert_temperature(true));
		}
		else if decision == 3 {
			print!("Specify Celsius temperature to convert: ");
			let _ = io::stdout().flush();
			println!("Converted temperature is {} degrees Farenheit!", convert_temperature(false));
		}
		else if decision == 4 {
			print!("Specify length in feet to convert: ");
			let _ = io::stdout().flush();
			println!("Converted length is {} meters!", convert_length(true));
		}
		else if decision == 5 {
			print!("Specify length in meters to convert: ");
			let _ = io::stdout().flush();
			println!("Converted length is {} feet!", convert_length(false));
		}
		else {
			println!("Goodbye!");
		}
	}
	Ok(())
}

fn obt() {
	print!("Specify trip distance: ");
	let _ = io::stdout().flush();
	let cin = io::stdin();
	let mut scanner = InputStream::new(cin.lock());
	let distance: i32 = scanner.scan().expect("An integer");
	let mut minutes = distance * 4 + 30;
	let mut hours = 0;
	if minutes >= 60 {
		hours = minutes / 60;
		minutes %= 60;
	}
	println!("Maximum on-board time is {} hours and {} minutes.", hours, minutes);
}

fn convert_temperature(do_metric: bool) -> f32 {
	let cin = io::stdin();
	let mut scanner = InputStream::new(cin.lock());
	let temp: f32 = scanner.scan().expect("An floating-point");
	let mut final_temp: f32 = 0.0;
	if do_metric {
		final_temp = (temp - 32.0) / 1.8;
	}
	else {
		final_temp = temp * 1.8 + 32.0;
	}
	final_temp
}

fn convert_length(do_metric: bool) -> f32 {
	let cin = io::stdin();
	let mut scanner = InputStream::new(cin.lock());
	let length: f32 = scanner.scan().expect("A floating-point");
	let mut final_length: f32 = 0.0;
	if do_metric {
		final_length = length  / 3.2808399;
	}
	else {
		final_length = length * 3.2808399;
	}
	final_length
}

fn menu() -> i32 {
	print!("Select an option:\r\n1) Metro Mobility Maximum On-Board Time\r\n2) Farenheit to Celsius\r\n3) Celsius to Farenheit\r\n4) Feet to Meters\r\n5) Meters to Feet\r\n6) Exit\r\nChoice: ");
	let _ = io::stdout().flush();
	let cin = io::stdin();
	let mut scanner = InputStream::new(cin.lock());
	let choice: i32 = scanner.scan().expect("An integer");
	choice
}
