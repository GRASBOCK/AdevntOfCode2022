use ascii::{AsciiChar, AsciiString};
use day05::parse_input;

fn apply_procedure(mut stacks: Vec<Vec<AsciiChar>>, procedure: &(usize, usize, usize)) -> Vec<Vec<AsciiChar>>{
    let (count, from, to) = procedure;
    let n_crates = stacks[*from].len();
    let (i_lowest, i_highest) = (n_crates - count, n_crates);
    //print!("{:?}, {:?}, {}, {}, {} ", &stacks[*from], &stacks[*to], count, i_lowest, i_highest);
    // how do I avoid cloning here? 
    // I know these are two separate stacks where the operation on one will not affect the other, but both are within one mutable variable
    let boxes_to_be_moved = Vec::from(&stacks[*from][i_lowest..i_highest]);
    stacks[*to].extend(boxes_to_be_moved);
    stacks[*from].resize(i_lowest, AsciiChar::Minus);
    //println!("--> {:?}, {:?}", &stacks[*from], &stacks[*to]);
    return stacks;
}

fn main(){  
    let (stacks, procedures) = parse_input();
    let mut stacks = stacks;
    for procedure in procedures.iter(){
        stacks = apply_procedure(stacks, &procedure);
    }
    let message = stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&AsciiChar::Space))
        .collect::<AsciiString>();
    println!("{message}");
}
