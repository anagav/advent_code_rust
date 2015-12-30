#[allow(dead_code)]

use std::io::{self,BufRead};

static VOWELS:[char;5] = ['a','e','i','o','u'];
static STR_EXCLUSIONS:[&'static str;4]= ["ab","cd","pq","xy"];


fn count_vowels(line: &str) -> bool{
    let mut count =0;
    for c in line.chars(){
        if VOWELS.contains(&c){
            count+=1;
        }
        if count >= 3{
            return true;
        }
    }
    return false;
}

fn contains_char_sequence(line: &str) -> bool{
    let mut current = ' ';
    for c in line.chars(){
        if current==c{
            return true;
        }
        current = c;
    }
    return false;
}

fn contains_invalid_sequence(line: &str) -> bool{
    for pat in STR_EXCLUSIONS.iter(){
        if line.contains(pat){
            return true;
        }
    }
    return false;
}



fn is_nice(line: &str)-> bool{
    if !count_vowels(&line){
        //println!("{:?}", "count vowels failed");
        return false;
    }
    if !contains_char_sequence(&line){
        //println!("{:?}", "char sequence failed");
        return false;
    }
    if contains_invalid_sequence(&line){
        //println!("{:?}", "invalid char sequence failed");
        return false;
    }
    return true;
}


pub fn run(){
    let stdin =  io::stdin();
    let mut count =0;
    for line in stdin.lock().lines(){
        if is_nice(&line.unwrap()){
            count+=1;
        }
    }
    println!("{}",count);

}

fn main() {
    run();
}
