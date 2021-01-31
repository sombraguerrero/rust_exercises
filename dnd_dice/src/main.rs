use rand::{thread_rng, Rng};
use std::io::{self, Write};
//This program uses the input-stream crate (v0.4.0 as of 1/30/21) as a flexible workaround for the limitation of Windows console apps not implementing buffer reads on non-UTF8 data
use input_stream::InputStream;
fn main() -> io::Result<()> {
	print!("Specify number of dice and rolls: ");
	let _ = io::stdout().flush(); // necessary because Rust does not flush output streams without newlines appended. (as of v1.51, 1/30/21)
	let cin = io::stdin();
	let mut scanner = InputStream::new(cin.lock());
	let dice: i32 = scanner.scan().expect("An integer");
	let sides: i32 = scanner.scan().expect("An integer");
	let mut rolls: Vec<i32> = Vec::new();
	let mut my_generator = thread_rng();
	for _elem in 0..dice {
		rolls.push(my_generator.gen_range(0..=sides));
	}
	let mut sum = 0;
	println!("Your rolls are: ");
	for elem in rolls {
		print!("{}  ", elem);
		sum += elem;
	}
	println!("\r\nYour total is {}.", sum);
Ok(())
}
