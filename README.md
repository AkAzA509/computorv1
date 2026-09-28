# Computorv1

Computorv1 is a small command-line polynomial solver written in Rust. It reduces an equation and solves it when its degree is 0, 1, or 2.

## Requirements

- Rust and Cargo

## Build

```bash
cargo build --release
```

The executable is created at `target/release/computorv1`.

## Usage

Pass the equation as the first argument:

```bash
cargo run -- "5 * X^0 + 4 * X^1 - 9 * X^2 = 1 * X^0"
```

You can also run the program without an argument and enter the equation when prompted:

```bash
cargo run
```

The input must contain exactly one `=`. Terms use the following format:

```text
coefficient * X^exponent
```

Terms and operators must be separated by spaces. For example:

```text
2 * X^2 + 3 * X^1 - 5 * X^0 = 0 * X^0
```

## How it works

The program first moves every term to the left-hand side and combines terms with the same exponent. It then determines the polynomial degree and applies the corresponding method.

For a first-degree equation written as

$$
ax + b = 0
$$

the solution is

$$
x = -\frac{b}{a}
$$

For a second-degree equation written as

$$
ax^2 + bx + c = 0
$$

the discriminant is

$$
\Delta = b^2 - 4ac
$$

The program handles the three possible cases:

- If $\Delta > 0$, there are two distinct real solutions:

$$
x_1 = \frac{-b - \sqrt{\Delta}}{2a}, \qquad
x_2 = \frac{-b + \sqrt{\Delta}}{2a}
$$

- If $\Delta = 0$, there is one real solution:

$$
x = \frac{-b}{2a}
$$

- If $\Delta < 0$, there are two complex solutions:

$$
x_1 = \frac{-b - i\sqrt{-\Delta}}{2a}, \qquad
x_2 = \frac{-b + i\sqrt{-\Delta}}{2a}
$$

Polynomials with a degree greater than 2 are identified but not solved.

## Examples

```bash
cargo run -- "2 * X^2 + 3 * X^1 - 5 * X^0 = 0 * X^0"
```

```text
Reduce form: -5 * X^0 + 3 * X^1 + 2 * X^2 = 0
Polynomial degree: 2
Discriminant is strictly positive, the two solutions are:
-2.5
1
```

## License

This project is distributed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for the full text.