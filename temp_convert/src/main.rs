// I've failed to make a command line menu for now.  Crappy loop with calc logic...

use std::io;

fn main() {
    loop {
        println!("Enter a temp in Celsius to convert to Fahrenheit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("{input} Celsius is {} Fahrenheit", c_to_f(input));

        println!("Enter a temp in Fahrenheit to convert to Celsius");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        println!("{input} Fahrenheit is {} Celsius", f_to_c(input));
    }
}

fn c_to_f(temp: f32) -> f32 {
    temp * (9.0 / 5.0) + 32.0
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * (5.0 / 9.0)
}
