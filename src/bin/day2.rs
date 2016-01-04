#[allow(non_snake_case)]
#[allow(dead_code)]

use std::io::{self,BufRead};

fn calculate_paper_needed(v: &mut Vec<i32>) -> i32 {
    let surface_area = 2*(v[0]*v[1] + v[1]*v[2] + v[2]*v[0]);
    v.sort();
    let additional_area = v[0]*v[1];
    return surface_area + additional_area;
}

fn ribbon_feed(v: &Vec<i32>) -> i32{
    (2*(v[0]+v[1])) + (v[0]*v[1]*v[2])
}

pub fn run(){
    let stdin =  io::stdin();
    let (paper_needed,paper_needed_2) = stdin.lock().lines().map(
        |line| line.unwrap()
    ).map(
        |line| {
            line.split("x").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        }
    ).filter_map(
        |mut vector| {
            Some((calculate_paper_needed(&mut vector),ribbon_feed(&vector)))
        }
    ).fold((0,0),|(sum,sum2),(element,element2)|{
        (sum+element,sum2+element2)
    });
    println!("problem-1 {:?}", paper_needed);
    println!("problem-2 {:?}", paper_needed_2);
}

fn main() {
    run();
}
