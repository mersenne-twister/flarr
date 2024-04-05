use flarr::Flarr;
use std::{
    fs::File,
    io::{self, read_to_string, Read},
};

fn main() {
    let mut vec = Vec::new();
    let mut my_immortal = String::new();
    File::open("my-immortal.txt")
        .unwrap()
        .read_to_string(&mut my_immortal)
        .unwrap();

    for str in my_immortal.split_ascii_whitespace() {
        vec.push(str);
    }
    let flarr = Flarr::new(vec);

    println!(
        "Welcome to the Flarr Demo!\n\
        Enter any float >= 0 & < 1 to index the contents of the my_immortal fanfiction!\n\
        WARNING: may include profanity, unseemly acts, and bad writing."
    );

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let Ok(index) = input.trim().parse::<f64>() else {
            continue;
        };
        if (index < 0.) || (index >= 1.) {
            continue;
        }

        println!("{}", flarr[index]);
    }
}
