use std::{env, io};

fn get_and_parse_param() -> (Result<(), Box<dyn std::error::Error>>, String)
{
	let args: Vec<String> = env::args().collect();
	let mut expr = String::new();

	if args.len() < 1 || args.len() > 2 {
		return (Err("Error: there are too many or not enough parameters".into()), String::new());
	} else if args.len() == 1 {
		io::stdin()
			.read_line(&mut expr)
			.expect("Error: reading inout from stdin");
	} else {
		expr = args[1].clone();
	}
	if expr.len() <= 0 {
		return (Err("Error: the expression is empty".into()), String::new());
	}

	expr = expr.trim().to_string();

	println!("Expression enter : {expr}");

	return (Ok(()), expr);
}

fn reduce_expr(expr : &String)
{
	let it : Vec<String> = expr.rsplit('=').map(str::trim).map(str::to_owned).collect();

	for i in it {
		println!("{i}");
	}
	
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
	let ret : (Result<(), Box<dyn std::error::Error>>, String) = get_and_parse_param();
	if ret.0.is_err() {
		return ret.0;
	}

	reduce_expr(&ret.1);

	return Ok(());
}