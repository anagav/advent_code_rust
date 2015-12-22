use std::io;
use std::io::prelude::*;


pub fn run() {
    //let mut map = HashMap::new();
    let mut input = String::new();
    let mut counter =0;

    io::stdin().read_line(&mut input).unwrap();

    //match io::stdin().read_line(&mut input){
    //      Ok(n) => {},
    //      Err(error) => println!("error: {}", error),
    //}

    for c in input.trim().chars() {
        match c {
            '(' => {
                counter+=1;
            }
            ')' => {
                counter-=1;
            }
            _ => {
                panic!("{}{}","for unexpected char:",c);
            }
        }
    }

    println!("counter: {}",counter);

}
