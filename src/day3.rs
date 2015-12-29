#[allow(non_snake_case)]
#[allow(dead_code)]

use std::io::{self,BufRead};
use std::collections::*;

#[derive(PartialEq, Eq, Hash,Debug,Clone,Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Point{
    fn incrX(&mut self){
        self.x+=1;
    }

    fn incrY(&mut self) {
        self.y+=1;
    }

    fn decrX(&mut self){
        self.x-=1;
    }

    fn decrY(&mut self) {
        self.y-=1;
    }

}



fn update_counter<'a>(point: Point, counter: &mut HashMap<Point, i32>){
    let count = counter.entry(point).or_insert(0);
    *count+=1;
}



pub fn run(){
    let stdin = io::stdin();
    let mut counter = HashMap::<_,_>::new();
    let mut start_point = Point{x:0,y:0};
    counter.entry(start_point).or_insert(1);
    for line in stdin.lock().lines(){
        for c in line.unwrap().chars(){
            match c {
                '^' =>{
                    start_point.incrY();
                    update_counter(start_point, &mut counter);
                }
                'v' => {
                    start_point.decrY();
                    update_counter(start_point, &mut counter);
                }
                '>' => {
                    start_point.incrX();
                    update_counter(start_point, &mut counter);
                }
                '<' => {
                    start_point.decrX();
                    update_counter(start_point, &mut counter);
                }
                _ => {
                    panic!("unexpected input: {}",c);
                }
            }
        }
        println!("{}", counter.len());
    }
}
