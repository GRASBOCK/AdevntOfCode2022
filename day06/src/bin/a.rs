use std::collections::HashSet;

use ascii::AsciiChar;
use day06::parse_input;

fn main(){
    
    let input = parse_input();
    for offset in 0..input.len(){
        let marker_index = offset + 4;
        let patch = &input[offset..marker_index];
        let set = patch.chars().collect::<HashSet::<AsciiChar>>();
        if set.len() == 4{
            println!("{}", marker_index);
            break;
        }
    }
}