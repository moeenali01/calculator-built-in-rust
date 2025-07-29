use std::io;
fn main(){
    println!("Calculator");
    println!("Available operations: + (addition), - (subtraction), * (multiplication), / (division)");
    println!("Enter operation: e.g 5 + 3");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    if tokens.len() !=3{
        println!("Invalid Input, Follow the structure: number operator number")
    }

    if tokens[1] == "+" {
        let num1: f64 = tokens[0].parse().unwrap();
        let num2: f64 = tokens[2].parse().unwrap();
        let result = add(num1, num2);
        println!("Result: {}", result);
    } else if tokens[1] == "-" {
        let num1: f64 = tokens[0].parse().unwrap();
        let num2: f64 = tokens[2].parse().unwrap();
        let result = subtract(num1, num2);
        println!("Result: {}", result);
    } else if tokens[1] == "*" {
        let num1: f64 = tokens[0].parse().unwrap();
        let num2: f64 = tokens[2].parse().unwrap();
        let result = multiply(num1, num2);
        println!("Result: {}", result);
    } else if tokens[1] == "/" {
        let num1: f64 = tokens[0].parse().unwrap();
        let num2: f64 = tokens[2].parse().unwrap();
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