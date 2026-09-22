use std::env;

fn parse_input(expr: &String) -> Result<(), Box<dyn std::error::Error>>
{
	if expr.len() <= 0 {
		return Err("Error: the expression is empty".into());
	}

	return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let args: Vec<String> = env::args().collect();

	if args.len() <= 1 || args.len() > 2 {
		return Err("Error: there are too many or not enough parameters".into());
	} else {
		let ret = parse_input(&args[1]);
		if ret.is_err() {
			return ret;
		}
	}

	return Ok(());
}