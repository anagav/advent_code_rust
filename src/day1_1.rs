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
    let mut position = 0;
    let mut first = true;
    for c in input.trim().chars() {
        position+=1;
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
        if (counter < 0 && first){
            first = false;
            println!("basement position: {}", position);
        }
    }

    println!("counter: {}",counter);

}
