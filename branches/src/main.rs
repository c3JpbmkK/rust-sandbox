use std::io;

fn main() {
    let number = String::new();

    loop {
        match io::stdin().read_line(&mut number) {
            Ok(input) => input,
            Err(err) => {
                println!("Error {}", err);
            }
        };

        let number = number.trim().parse();

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
}
