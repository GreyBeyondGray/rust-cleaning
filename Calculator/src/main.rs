use std::io;

fn main() {
    println!("Welcome to this simple calculator.");

    println!("Please choose your option.");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to get input.");

    let choice: i32 = choice.trim().parse().expect("Not an integer");

    println!("Please input first number:");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("must be an integer");

    let x: i32 = x.trim().parse().expect("Must be an integer");

    println!("Please input second number:");

    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("must be an integer");

    let y: i32 = y.trim().parse().expect("Must be an integer");

    match choice {
        1 => {
            let result: i32 = add(x, y);
            println!("{} + {} = {}", x, y, result);
        }
        2 => {
            let result: i32 = subtract(x, y);
            println!("{} - {} = {}", x, y, result);
        }
        3 => {
            let result: i32 = multiply(x, y);
            println!("{} * {} = {}", x, y, result);
        }
        4 => {
            let result: i32 = divide(x, y);
            println!("{} / {} = {}", x, y, result);
        }
        _ => println!("Dunno"),
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn divide(x: i32, y: i32) -> i32 {
    x / y
}
