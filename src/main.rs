use std::cmp::Ordering;
use rand::Rng;

fn make_guess(min: u64, max: u64) -> u64{
    return (min + max) / 2;
}

fn main() {

    let mut min = 1;
    let mut max = 100;
    let count = 0;
    let secret_number = rand::thread_rng().gen_range(min, max+1);
    println!("Secret number is: {}.", secret_number);
    
    loop {
        let count = count + 1;
        let guess = make_guess(min, max);

        println!("Binary search guess #{} is: {}.", count, guess);
        

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                min = guess;
            }
            Ordering::Greater => {
                println!("Too big!");
                max = guess;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
