use std::char;
use itertools::Itertools;
fn priority(c: char) -> u32{
    let ascii_val = c as u32;
    if 96 < ascii_val && ascii_val < 123{
        // a to z
        return ascii_val - 96;
    }
    if 64 < ascii_val && ascii_val < 91{
        return ascii_val - 64 + 26;
    }
    panic!("c {c} is not a valid character");
}

fn badge(group: (&str, &str, &str)) -> char{
    for char in group.0.chars(){
        if group.1.contains(char) && group.2.contains(char){
            return char
        }
    }
    panic!("no badge found")
}

fn main() {    
    let score = include_str!("../../input.txt")
        .lines()
        .tuples()
        .fold(0, |acc, (line1, line2, line3)|{
            let badge = badge((line1, line2, line3));
            acc + priority(badge)
        });
    println!("{score}");
}
