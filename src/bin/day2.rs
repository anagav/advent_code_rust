#[allow(non_snake_case)]
#[allow(dead_code)]

use std::io::{self,BufRead};

fn calculate_paper_needed(v: &mut Vec<i32>) -> i32 {
    let surface_area = 2*(v[0]*v[1] + v[1]*v[2] + v[2]*v[0]);
    //println!("{}",surface_area);
    v.sort();
    let additional_area = v[0]*v[1];
    return surface_area + additional_area;
}



pub fn run(){
    let stdin =  io::stdin();
    let mut paper_needed=0;
    let result:Vec<_> = stdin.lock().lines().map(
        |x| x.unwrap().split("x").map(
            |y| y.parse::<i32>().unwrap()
        ).collect::<Vec<i32>>()
    ).map(|in_vec|{
        calculate_paper_needed(&mut in_vec)
    });

    println!("{:?}", result);



    //
    // {
    //     //println!("{}",line);
    //     let input = line.unwrap();
    //     let mut in_line:Vec<_> = input.split("x").map(
    //         |x| x.parse::<i32>().unwrap()
    //     )
    //
    //     println!("{:?}", in_line);
    //
    //     paper_needed += calculate_paper_needed(&mut in_line);
    // }
    // println!("{}",paper_needed);
}

fn main() {
    run();
}
