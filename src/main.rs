use std::io;
use std::io::Write;

use inquire::Select;

mod calculator;
mod read_file;
mod todo_list;

fn main() {

    /*
    Fisrt run: Show selection to choose
    -> Calculator
        > Calculator Operations ex: 2*19-12+13/2...
    -> Todo-List
        > Add Task
        > Delete Task
    -> Read file
     */

    // use library dependencies
    option_use_inquire();

    // // with native code
    // option_prompt_native();
}

fn option_prompt_native() {
    let options = ["Calculator", "Todo-List", "Read File"];
    
    println!("What do you want ?");
    for (i, opt) in options.iter().enumerate() {
        println!("{} ) {}", i + 1, opt);
    }
    
    print!("Enter choice [1-{}]: ", options.len());
    io::stdout().flush().unwrap();
    
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    match buf.trim().parse::<usize>() {
        Ok(n) if n >= 1 && n <= options.len() => {
            match options[n - 1] {
                "Calculator" => calculator::run_calculator(),
                "Todo-List"  => todo_list::open_todo_options(),
                "Read File"  => read_file::open_and_read_file().unwrap(),
                _ => {}
            }
        }
        _ => eprintln!("Invalid selection"),
    }
}

fn option_use_inquire() {
    let options: Vec<&str> = vec!["Calculator", "Todo-List", "Read File"];

    let choice = Select::new("What do you want ?", options)
        .with_help_message("Use ↑/↓ and Enter")
        .prompt();

    match choice {
        Ok("Calculator") => calculator::run_calculator(),
        Ok("Todo-List")  => todo_list::open_todo_options_with_inquire(),
        Ok("Read File")  => read_file::open_and_read_file().unwrap(),
        Ok(_) => {},
        Err(e) => println!("Canceled or error: {}", e)
    }
}