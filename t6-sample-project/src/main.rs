//use io::stdin;
use rand::Rng;
use std::io::stdin;

fn main() {
    // Make random guess
    let mut rng = rand::thread_rng();
    let rand_float: f64 = rng.gen::<f64>() * 100.0;
    let the_number = format!("{:.0}",rand_float);
    let the_number: u8 = the_number.parse().unwrap();

    println!("{}",the_number);

    loop{
    // Take user input
    let mut guess = String::new();
    stdin().read_line(&mut guess).unwrap();

    // Trim and get number
    let guess_num: u8 = guess.trim_end().parse().unwrap();
    println!("You guess {}",guess_num);

    // Compare with random guess
    if guess_num < the_number {
        println!("Your number is smaller.");
    }
    else if guess_num > the_number {
        println!("Your number is larger.");
    }
    else if guess_num == the_number {
        println!("Congrats! You guessed it.");
        break;
    }
    }
}
