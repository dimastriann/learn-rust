use::std::io;

mod calculator;

fn main() {
    println!("Rust Expression Calculator");
    println!("Enter an expression (example: 10 + 5 * 2 - (4 / 2)):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let result = calculator::evaluate(input.trim());
    println!("Result: {}", result);
}
