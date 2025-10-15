use std::io;

fn main() {
    println!("temperature converter!!");
    println!("1 Celcius to Fahrenheit");
    println!("2 Fahrenheit to Celcius");
    println!("choose one of the options above(1 or 2): ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read line");

    // string -> u32に変換する
    let choice = match choice.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input 1, 2");
            return;
        }
    };

    println!("user input: {}", choice);

    let NumOption = handle_input_for_cal();
    let num = match NumOption {
        Some(num) => num,
        Err(_) => 0.0,
    }
}

fn handle_input_for_cal() -> Option<f64> {
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("failed to read input. can't calculate!");

    let temp = match temp.trim().parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Not number");
            return None;
        }
    };
    return temp;
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    let fahrenheit = celcius * 1.8 + 32.0;
    fahrenheit
}
