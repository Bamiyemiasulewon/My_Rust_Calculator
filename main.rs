Calculator.
use std::io;

fn main() {
    loop {
        println!("Enter an expression (e.g., 2 + 3, type 'exit' to quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.to_lowercase() == "exit" {
            println!("Exiting the calculator. Goodbye!");
            break;
        }

        match evaluate_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn evaluate_expression(expr: &str) -> Result<f64, &'static str> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();

    if tokens.len() != 3 {
        return Err("Invalid expression. Please use the format: number operator number");
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(value) => value,
        Err(_) => return Err("Invalid first number"),
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(value) => value,
        Err(_) => return Err("Invalid second number"),
    };

    match operator {
        "+" => Ok(add(num1, num2)),
        "-" => Ok(subtract(num1, num2)),
        "*" => Ok(multiply(num1, num2)),
        "/" => divide(num1, num2),
        _ => Err("Invalid operator. Supported operators are +, -, *, /"),
    }
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b != 0.0 {
        Ok(a / b)
    } else {
        Err("Division by zero is not allowed")
    }
}
