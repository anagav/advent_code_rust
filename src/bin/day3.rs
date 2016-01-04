#[allow(non_snake_case)]
#[allow(dead_code)]

use std::io::{self,BufRead,Read};
use std::collections::*;

pub fn run(){
    let stdin = io::stdin();
    let mut start_point = (0,0);
    let result:Vec<_> = stdin.lock().lines().map(
        |line| line.unwrap()
    ).filter_map(
        |line| {
            let result = line.chars().map(
                |char_c| {
                    match char_c {
                        '^' => (0,1),
                        'v' => (0,-1),
                        '<' => (-1,0),
                        '>' => (1,0),
                        _ => panic!("[PANIC] got invalid char: {}", char_c)
                    }
                }
            ).collect::<Vec<_>>();
            Some(result)
        }
    ).collect();

    let mut results = HashSet::<_>::new();
    let mut santa_houses_only = HashSet::<_>::new();
    let mut robo_santa_houses = HashSet::<_>::new();

    results.insert((0,0));


    for vector in &result{
        for mov in vector {
            start_point = (start_point.0+mov.0,start_point.1+mov.1);
            results.insert(start_point);
        }
        start_point = (0,0);
    }

    let mut is_robo_santa = false;
    let mut robo_santa_start_point = (0,0);
    let mut santa_start_point = (0,0);

    for vector in &result{
        for mov in vector {
            if is_robo_santa{
                robo_santa_start_point = (robo_santa_start_point.0 + mov.0, robo_santa_start_point.1 + mov.1);
                robo_santa_houses.insert(robo_santa_start_point);
            }else{
                santa_start_point = (santa_start_point.0 + mov.0, santa_start_point.1 + mov.1);
                santa_houses_only.insert(santa_start_point);
            }
            is_robo_santa = !is_robo_santa;
        }
    }

    println!("santa houses {:?}", santa_houses_only.len());
    println!("robo santa houses {:?}", robo_santa_houses.len());
    let houses:Vec<_> = santa_houses_only.union(&robo_santa_houses).collect();
    println!("part 1 results: {:?}", results.len());
    println!("part 2 results: {:?}", houses.len());
}


fn main() {
    run();
}
