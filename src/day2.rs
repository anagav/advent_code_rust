


fn calculate_paper_needed(length: &i32, width: &i32, height: &i32) -> i32 {
    let surface_area = 2*(length*width + width*height + height*length);
    let mut v = [length,width,height];
    v.sort();
    let additional_area = v[0]*v[1];
    return surface_area + additional_area;
}



pub fn run(){
    let paper_needed = calculate_paper_needed(&1,&2,&1);
    println!("{}",paper_needed);
}
