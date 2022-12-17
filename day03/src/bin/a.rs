use std::char;

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

fn misplaced_item(compartment1: &str, compartment2: &str) -> char{
    for char in compartment1.chars(){
        if compartment2.contains(char){
            return char
        }
    }
    panic!("no item misplaced")
}

fn main() {    
    let score = include_str!("../../input.txt")
        .lines()
        .fold(0, |acc, line|{
            let index_half = line.len()/2;
            let compartment1 = &line[0..index_half];
            let compartment2 = &line[index_half..];
            let misplaced_item = misplaced_item(compartment1, compartment2);
            acc + priority(misplaced_item)
        });
    println!("{score}");
}
