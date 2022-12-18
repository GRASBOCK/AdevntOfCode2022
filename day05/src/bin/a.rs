use ascii::{AsciiChar, AsciiString};
use day05::parse_input;

fn apply_procedure(mut stacks: Vec<Vec<AsciiChar>>, procedure: &(usize, usize, usize)) -> Vec<Vec<AsciiChar>>{
    let (count, from, to) = procedure;
    for _ in 0..*count{
        let id = stacks[*from].pop().expect("trying to remove more crates than there are available");
        stacks[*to].push(id);
    }
    return stacks;
}

fn main(){  
    let (stacks, procedures) = parse_input();
    let mut stacks = stacks;
    for procedure in procedures{
        stacks = apply_procedure(stacks, &procedure);
    }
    let message = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<AsciiString>();
    println!("{message}");
}
