use std::io;

mod lib;
use lib::*;

fn main() {
    println!("{}", is_prime(17));

}

/*
fn main() -> io::Result<()> {
    println!("Enter a number:");
    let mut num = String::new();
    io::stdin().read_line(&mut num)?;
    let num: u128 = match num.trim().parse() {
        Ok(o) => o,
        Err(e) => panic!("Enter a number! ERROR: {}", e),
    };
    match is_prime(num, 2){
        true =>    println!("{} is prime", num),
        false =>    println!("{} is not prime", num),
    }

    Ok(())
}
*/
