use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let s = rand::thread_rng().gen_range(1..101);
    println!("{}", s);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fuck");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("number you stupid");
                continue;
            }
        };

        println!("{} {}", guess, s);

        match guess.cmp(&s) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("yes");
                break;
            }
        }
    }
}
