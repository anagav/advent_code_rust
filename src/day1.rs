#[allow(non_snake_case)]
#[allow(dead_code)]

use std::io;
use std::io::prelude::*;


pub fn run() {
    //let mut map = HashMap::new();
    let mut input = String::new();
    let mut counter =0;

    io::stdin().read_line(&mut input).unwrap();
    let mut position = 0;
    let mut first = true;

    let (count,basement) = input.trim().chars().map(
        |x| match x{
            '(' => 1,
            ')' => -1,
            _ => panic!("invalid input: {}",x)
        }
    ).enumerate().fold(
        (0,None), |(count,basement),(elem_pos,elem)| {
            let temp_count = count+elem;
            (temp_count, if temp_count == -1 { basement.or(Some(elem_pos+1)) } else {basement})
        }
    );

    println!("counter: {:?} \nbasement: {:?}", count,basement.unwrap_or_else(|| 0));
}
