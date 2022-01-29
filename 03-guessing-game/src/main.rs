use rand::Rng;
use std::io;

fn main() {
  let secret_number = rand::thread_rng()
      .gen_range(1..101);

  println!("Secret number {}", secret_number);

  println!("Guess the number!");
  println!("Please input your guess.");
  
  let mut guess = String::new();
  
  io::stdin()
    .read_line(&mut guess)
    .expect("failed to read the line");
  
  
  println!("You guessed: {}", guess);
}
