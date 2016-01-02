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

fn ribbon_feed(v: &mut Vec<i32>) -> i32{
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
        |mut vector|{
            Some((calculate_paper_needed(&mut vector),ribbon_feed(&mut vector)))
        }
    ).fold((0,0),|(sum,sum2),(element,element2)|{
        (sum+element,sum2+element2)
    });

    println!("problem-1 {:?}", paper_needed);
    println!("problem-2 {:?}", paper_needed_2);


    // let result:Vec<_> = stdin.lock().lines().map(
    //     |x| x.unwrap().split("x").map(
    //         |y| y.parse::<i32>().unwrap()
    //     ).collect::<Vec<i32>>()
    // ).map(|in_vec|{
    //     calculate_paper_needed(&mut in_vec)
    // });

    //println!("{:?}", result);



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
