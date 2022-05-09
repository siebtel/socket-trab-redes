use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..101);  
  let mut guess = String::new();
  
  loop {
    println!("Please input your guess.");

    
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
    let guess: u32 = guess.trim().parse().expect("Failed to parse");

    // let guess: u32 = match guess.trim().parse() {
    //   Ok(num) => num,
    //   Err(_) => {
    //     println!("Please input a number!");
    //     continue;
    //   }
    // };
    
    
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      },
    }
  }
}