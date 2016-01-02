// #[allow(non_snake_case)]
// #[allow(dead_code)]
//
// use std::io::{self,BufRead,Read};
// use std::collections::*;
//
// #[derive(PartialEq, Eq, Hash,Debug,Clone,Copy)]
// struct Point {
//     x: i32,
//     y: i32
// }
//
// impl Point{
//     fn incr_x(&mut self){
//         self.x+=1;
//     }
//
//     fn incr_y(&mut self) {
//         self.y+=1;
//     }
//
//     fn decr_x(&mut self){
//         self.x-=1;
//     }
//
//     fn decr_y(&mut self) {
//         self.y-=1;
//     }
//
// }
//
//
//
// fn update_counter<'a>(point: Point, counter: &mut HashMap<Point, i32>){
//     let count = counter.entry(point).or_insert(0);
//     *count+=1;
// }
//
//
//
// pub fn run(){
//     let stdin = io::stdin();
//     let mut counter = HashMap::<_,_>::new();
//     let mut start_point = Point{x:0,y:0};
//     counter.entry(start_point).or_insert(1);
//
//     let result:Vec<_> = stdin.chars().map(
//         |x|{
//             match x.unwrap() {
//                 '^' =>{
//                     start_point.incr_y();
//                     update_counter(start_point, &mut counter);
//                 }
//                 'v' => {
//                     start_point.decr_y();
//                     update_counter(start_point, &mut counter);
//                 }
//                 '>' => {
//                     start_point.incr_x();
//                     update_counter(start_point, &mut counter);
//                 }
//                 '<' => {
//                     start_point.decr_x();
//                     update_counter(start_point, &mut counter);
//                 }
//                 _ => {
//                     panic!("unexpected input:");
//                 }
//             }
//         }
//     ).collect();
//     println!("{}", counter.len());
// }
//
//
// fn main() {
//     run();
// }
