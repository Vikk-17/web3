// write a code to generate sha 256 provided an input string
// and the output sha256 should start with 00000

// use std::io;

// fn main() {
//     let mut input = String::new();
//     println!("Enter your input: ");
//
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//
//     println!("Hello, {}!", input.trim()); // Trim removes trailing newline
// }


use hex_literal::hex;
use sha2::{Sha256, Digest};

fn findHashWithPrefix(prefix: u8){
    let mut input = 0;
    loop {
        let mut inputStr = input.to_string();


        input += 1;
    }
}


fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"Carmack");
    let result:Vec<_> = hasher.finalize().to_vec();
    println!("{:?}", result.iter().collect());
}
