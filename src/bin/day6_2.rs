use std::io::{self,BufRead};


fn turn_on_lights(array:&mut Vec<Vec<i32>>, start_x:&usize,start_y:&usize,end_x:&usize,end_y:&usize){
    for i in *start_x..*end_x{
        for j in *start_y..*end_y{
            //println!("{:?}", array[i]);
            array[i][j]+=1;
        }
    }
}

fn turn_off_lights(array:&mut Vec<Vec<i32>>, start_x:&usize,start_y:&usize,end_x:&usize,end_y:&usize){
    for i in *start_x..*end_x{
        for j in *start_y..*end_y{
            array[i][j]-=1;
            if array[i][j]<0{
                array[i][j]=0;
            }
        }
    }
}

fn toggle_lights(array:&mut Vec<Vec<i32>>, start_x:&usize,start_y:&usize,end_x:&usize,end_y:&usize){
    for i in *start_x..*end_x{
        for j in *start_y..*end_y{
            array[i][j]+=2;
        }
    }
}

fn perform_operation(line:&str, array:&mut Vec<Vec<i32>>){
    let splits:Vec<_> = line.split(" ").collect::<Vec<&str>>();

    if line.starts_with("toggle"){
        let start_x:usize = splits[1].split(",").collect::<Vec<&str>>()[0].parse().unwrap();
        let start_y:usize = splits[1].split(",").collect::<Vec<&str>>()[1].parse().unwrap();
        let end_x:usize = splits[3].split(",").collect::<Vec<&str>>()[0].parse().unwrap();
        let end_y:usize = splits[3].split(",").collect::<Vec<&str>>()[1].parse().unwrap();
        toggle_lights(array,&start_x,&start_y,&(end_x+1),&(end_y+1));
        return;
    }


    let start_x:usize = splits[2].split(",").collect::<Vec<&str>>()[0].parse().unwrap();
    let start_y:usize = splits[2].split(",").collect::<Vec<&str>>()[1].parse().unwrap();
    let end_x:usize = splits[4].split(",").collect::<Vec<&str>>()[0].parse().unwrap();
    let end_y:usize = splits[4].split(",").collect::<Vec<&str>>()[1].parse().unwrap();

    if line.starts_with("turn on"){
        turn_on_lights(array,&start_x,&(start_y),&(end_x+1),&(end_y+1));
    }
    if line.starts_with("turn off"){
        turn_off_lights(array,&start_x,&(start_y),&(end_x+1),&(end_y+1));
    }

}


fn init_vec(array:&mut Vec<Vec<i32>>){
    for _i in 0..1000{
        let mut inner_array = Vec::<i32>::new();
        for _j in 0..1000{
            inner_array.push(0);
        }
        array.push(inner_array);
    }
}

fn count_num_lights_on(array:&Vec<Vec<i32>>) -> i32{
    let mut count =0;
    for vector in array{
        for num in vector{
            count+=*num;
        }
    }
    return count;
}


pub fn run(){
    let stdin = io::stdin();
    let mut array:Vec<Vec<i32>> = Vec::new();
    init_vec(&mut array);
    for line in stdin.lock().lines(){
        perform_operation(&line.unwrap(),&mut array);
    }

    println!("lights set: {}", count_num_lights_on(&array));
}

fn main() {
    run();
}
