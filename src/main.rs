use std::io;
///Parth Patel
/// Rust final Project : This project is a Simple calulator which will take users input adn the opertion they want to choose
/// Attributes: This is my original work and no help is recieved creating this 
/// 
/// Author (Parth Patel)
/// Date(Decemeber 18 2022)
/// 
fn main() {
    println!("Welcome to the calculator!");

    loop {
        println!("Enter an operator (+, -, *, /) and two operands:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: Vec<&str> = input.split_whitespace().collect();

        if input.len() != 3 {
            println!("Invalid input. Please try again.");
            continue;
        }

        let op = input[0];
        let x = input[1].parse::<f64>().expect("Failed to parse operand");
        let y = input[2].parse::<f64>().expect("Failed to parse operand");

        let result = match op {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => x / y,
            _ => {
                println!("Invalid operator. Please try again.");
                continue;
            }
            
        };

        println!("Result: {}", result);
    }
}