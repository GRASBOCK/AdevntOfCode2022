use ascii::{AsAsciiStr, AsciiChar};

fn parse_crates(start_string: &str) -> Vec<Vec<AsciiChar>>{
    let mut stacks = vec![Vec::<AsciiChar>::new(); 9];
    let start_string = start_string.as_ascii_str().unwrap();
    for line in start_string.lines(){
        for i in 0..9{
            let offset = 1 + i*4;
            let id = line[offset];
            if id.is_alphabetic(){
                stacks[i].push(id);
            }
        }
    }
    for i in 0..stacks.len(){
        stacks[i].reverse();
    }
    return stacks;
}

fn parse_procedures(procedures_string: &str) -> Vec<(usize, usize, usize)>{
    procedures_string
        .lines()
        .map(|line|{
            let instructions = line
                .split_whitespace()
                .collect::<Vec<&str>>();
            let count: usize = instructions[1].parse().expect(format!("wasn't able to parse as digit: {}", instructions[1]).as_str());
            let from = instructions[3].parse::<usize>().unwrap() - 1;
            let to = instructions[5].parse::<usize>().unwrap() - 1;
            (count, from, to)
        })
        .collect::<Vec<(usize, usize, usize)>>()
}

pub fn parse_input() -> (Vec<Vec<AsciiChar>>, Vec<(usize, usize, usize)>){
    let input = include_str!("../input.txt");
    let end_of_start_index = input.find("\n 1").unwrap();
    let start_string = &input[0..end_of_start_index];
    let stacks = parse_crates(start_string);

    let procedures_index = input[end_of_start_index..].find("move").unwrap() + end_of_start_index;
    let procedures_string = &input[procedures_index..];
    let procedures = parse_procedures(procedures_string);
    (stacks, procedures)
}