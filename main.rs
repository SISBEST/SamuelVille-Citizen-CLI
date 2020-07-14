extern crate curl;
use curl::http;
fn main() {
	let args: Vec<String> = std::env::args().collect();
  match args[1] {
		_ => println!("samuelville: Command not found...")
	};
}