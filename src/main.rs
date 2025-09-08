// use::std::io;

mod calculator;
mod read_file;
mod todo_list;

fn main() {
    println!("Rust Expression Calculator");
    println!("Enter an expression (example: 10 + 5 * 2 - (4 / 2)):");

    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();

    // let result = calculator::evaluate(input.trim());
    // println!("Result: {}", result);

    let content = read_file::open_and_read_file();
    println!("File Result: {:?}", content);

    let new_string = String::from("string");
    let string2 = &new_string;
    println!("owner: {}", new_string);
    println!("owner: {}", string2);

    let _ = todo_list::store_task();
}
