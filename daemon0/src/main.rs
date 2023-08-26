use std::io::stdin;

fn main() {
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => break,
            _ => print!("{}", input.to_uppercase())
        }
    }
    ()
}
