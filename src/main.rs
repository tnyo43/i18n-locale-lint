use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: my_cli_tool <arg1> <arg2>");
        return;
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

    println!("Argument 1: {}", arg1);
    println!("Argument 2: {}", arg2);
}
