use std::io;

fn main() {
    let mut i = String::new();
    io::stdin()
        .read_line(&mut i)
        .expect("fuck");

    println!("{}", i);
}
