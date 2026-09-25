use std::{collections::HashMap, env, error::Error, io};
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Term = (f64, u32);
type Polynomial = HashMap<u32, f64>;

fn get_param() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    let mut expr = args.get(1).cloned().unwrap_or_default();

    if args.len() > 2 {
        return Err("Error: there are too many or not enough parameters".into());
    }
    if args.len() == 1 {
    	print!("Enter an polynomial expression: ");
     	io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut expr)?;
    }

    let expr = expr.trim().to_owned();
    if expr.is_empty() {
        return Err("Error: the expression is empty".into());
    }

    // println!("Expression enter : {expr}");

    return Ok(expr);
}

fn parse_exponent(token: &str) -> Result<u32> {
    let (_, exponent) = token
        .split_once('^')
        .ok_or_else(|| format!("Error: invalid term format '{}'", token))?;
    let exp = exponent
        .parse()
        .map_err(|_| format!("invalid exponent in term '{}'", token))?;
    Ok(exp)
}

fn parse_term(tokens: &[&str], sign: f64) -> Result<Term> {
    if tokens.len() != 3 || tokens[1] != "*" {
        return Err("Error: invalid term format".into());
    }

    let coefficient = tokens[0]
        .parse::<f64>()
        .map_err(|_| format!("Error: invalid coefficient '{}'", tokens[0]))?;
    let exponent = parse_exponent(tokens[2])?;

    Ok((sign * coefficient, exponent))
}

fn parse_terms(tokens: &[&str]) -> Result<Vec<Term>> {
    let mut terms = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let (sign, term_start) = if i == 0 {
            (1.0, i)
        } else {
            let sign = match tokens[i] {
                "+" => 1.0,
                "-" => -1.0,
                token => return Err(format!("Error: invalid sign '{}'", token).into()),
            };
            (sign, i + 1)
        };

        let term_end = term_start + 3;
        if term_end > tokens.len() {
            return Err("Error: invalid term format".into());
        }
        terms.push(parse_term(&tokens[term_start..term_end], sign)?);
        i = term_end;
    }

    Ok(terms)
}

fn merge_terms(left: &[Term], right: &[Term]) -> Polynomial {
    let mut coeffs = Polynomial::new();

    for (terms, sign) in [(left, 1.0), (right, -1.0)] {
        for &(coeff, exp) in terms {
            *coeffs.entry(exp).or_insert(0.0) += sign * coeff;
        }
    }

    coeffs
}

fn print_reduced_form(coeffs: &Polynomial) {
    let mut exponents: Vec<&u32> = coeffs.keys().collect();
    exponents.sort();
    print!("Reduce form: ");
    let mut first = true;
    for exp in exponents {
        let coeff = coeffs[exp];
        if first {
            print!("{} * X^{}", coeff, exp);
        } else if coeff < 0.0 {
            print!(" - {} * X^{}", -coeff, exp);
        } else {
            print!(" + {} * X^{}", coeff, exp);
        }
        first = false;
    }
    println!(" = 0");
}

fn solve_degree_2(a: f64, b: f64, c: f64) {
    println!("Polynomial degree: 2");

    let delta = b * b - 4.0 * a * c;

    if delta < 0.0 {
        println!("Discriminant is strictly negative, the two complex solutions are: ");
        let real_part = -b / (2.0 * a);
        let imaginary_part = (-delta).sqrt() / (2.0 * a);

        println!("{} + {}i", real_part, imaginary_part);
        println!("{} - {}i", real_part, imaginary_part);
    } else if delta == 0.0 {
        println!("The solution is : {}", -b / (2.0 * a));
    } else {
        println!("Discriminant is strictly positive, the two solutions are: ");
        let div = 2.0 * a;
        println!("{}", (-b - delta.sqrt()) / div);
        println!("{}", (-b + delta.sqrt()) / div);
    }
}

fn solve_degree_1(a: f64, b: f64) {
    println!("Polynomial degree: 1");
    let x = -b / a;
    println!("The solution is : {x}");
}

fn solve_degree_0(c: f64) {
    if c == 0.0 {
        println!("Any real number is a solution.");
    } else {
        println!("No solution.");
    }
}

fn parse_side(tokens: &[&str]) -> Result<Vec<Term>> {
    if let [value] = tokens {
        let value = value
            .parse::<f64>()
            .map_err(|_| format!("Error: invalid value '{}'", value))?;
        return Ok((value != 0.0).then_some((value, 0)).into_iter().collect());
    }
    parse_terms(tokens)
}

fn coefficient(polynomial: &Polynomial, exponent: u32) -> f64 {
    polynomial.get(&exponent).copied().unwrap_or(0.0)
}

fn compute(expr: &str) -> Result<()> {
    let (left, right) = expr
        .split_once('=')
        .ok_or("Error: expression must contain exactly one '='")?;
    if right.contains('=') {
        return Err("Error: expression must contain exactly one '='".into());
    }

    let left_tokens: Vec<&str> = left.split_whitespace().collect();
    let right_tokens: Vec<&str> = right.split_whitespace().collect();
    let left_expr = parse_side(&left_tokens)?;
    let right_expr = parse_side(&right_tokens)?;

    // reduce form
    let reduced = merge_terms(&left_expr, &right_expr);
    print_reduced_form(&reduced);

    let degree_max = reduced.keys().max().copied().unwrap_or(0);

    // resolve
    match degree_max {
        0 => solve_degree_0(coefficient(&reduced, 0)),
        1 => solve_degree_1(coefficient(&reduced, 1), coefficient(&reduced, 0)),
        2 => solve_degree_2(
            coefficient(&reduced, 2),
            coefficient(&reduced, 1),
            coefficient(&reduced, 0),
        ),
        _ => println!("Polynomial degree: {degree_max}\nThe polynomial degree is strictly greater than 2, I can't solve."),
    }
    Ok(())
}

fn main() -> Result<()> {
    compute(&get_param()?)
}
