use std::io::{stdin, self, Write};

fn main() {
	// print user input
    print!("How many numbers to dispaly: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            let _input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
        }
        Err(_) => println!("Failed to read line"),
    }
    let input = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    for i in 1..input+1 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        }
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }
}