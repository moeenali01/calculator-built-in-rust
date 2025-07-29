use std::io;
fn main(){
    println!("Simple Calculator");
    println!("Available operations: + (addition), - (subtraction), * (multiplication), / (division)");
    println!("Enter operation: e.g 5 + 3");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    if tokens.len() !=3{
        println!("Invalid Input, Follow the structure: number operator number")
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid first number");
            return;
        }
    };
    let num2: f64 = match tokens[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid second number");
            return;
        }
    };

    let operator = tokens[1];

    if operator == "+" {
        let result = add(num1, num2);
        println!("Result: {}", result);
    } else if operator == "-" {
q        let result = subtract(num1, num2);
        println!("Result: {}", result);
    } else if operator == "*" {
        let result = multiply(num1, num2);
        println!("Result: {}", result);
    } else if operator == "/" {
        let result = divide(num1, num2);
        println!("Result: {}", result);
    } else {
        println!("Invalid operator");
    }
}

fn add(num1: f64, num2: f64) -> f64 {
    return num1 + num2 
}

fn subtract(num1: f64, num2: f64) -> f64 {
    return num1 - num2;
}

fn multiply(num1: f64, num2: f64) -> f64 {
    return num1 * num2;
}

fn divide(num1: f64, num2: f64) -> f64 {
    if num2 != 0.0 {
        return num1 / num2;
    } else {
        println!("Division by zero error");
        return 0.0;
    }
}


