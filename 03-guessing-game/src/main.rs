use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  let secret_number = rand::thread_rng()
      .gen_range(1..101);

  println!("Secret number {}", secret_number);
  println!("Guess the number!");

  let mut attempts: u32 = 0;

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read the line");

    let guess: u32 = match guess.trim().parse() {
      Ok(number) => {
        attempts += 1;
        number
      },
      Err(_) => {
        println!("Wrong input! Please, input only positive numbers");
        continue;
      },
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small"),
      Ordering::Greater => println!("Too big"),
      Ordering::Equal => {
        println!("You win! The secret number was {} and you guessed it in {} attempts", secret_number, attempts);
        break;
      }
    }
  }
}
