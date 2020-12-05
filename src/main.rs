use std::io::BufRead;

fn main() {
    let array = ["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];
    let mut rem;
    let mut x = "".to_string();
    println!("Ingresa un numero: ");
    let mut dec: usize = std::io::stdin()
    .lock()
    .lines()
    .next()
    .expect("stdin should be available")
    .expect("couldn't read from stdin")
    .trim()
    .parse()
    .expect("input was not an integer");

    while dec > 0 {
        rem = dec % 16;
        let xy = array[rem].to_string() + &x;
        x = xy;
        dec = dec/16;
    }

    print!("{}", x);
}
