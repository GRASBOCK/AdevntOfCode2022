use day06::{parse_input, find_marker};

fn main(){
    
    let input = parse_input();
    println!("{}", find_marker(&input, 4));
}