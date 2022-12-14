use std::io;

fn main() {
    loop {
    // prompt user for input
    println!("Enter your calculation (e.g. 2 + 2):");

        // read user input
        let mut calculation = String::new();
        io::stdin()
            .read_line(&mut calculation)
            .expect("Failed to read line");

        // trim leading/trailing whitespace and split input on space
        let calculation: Vec<&str> = calculation.trim().split(" ").collect();

        // check if user entered valid input (must have 3 items: number, operator, number)
        if calculation.len() != 3 {
            println!("Invalid input. Please try again.");
            continue;
        }

        // convert first and third items to numbers
        let num1: f64 = calculation[0].parse().expect("Invalid number");
        let num2: f64 = calculation[2].parse().expect("Invalid number");

        // match on operator and perform calculation
        let result = match calculation[1] {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => {
                println!("Invalid operator. Please try again.");
                continue;
            }
        };

        // print result
        println!("Result: {}", result);
    }
}