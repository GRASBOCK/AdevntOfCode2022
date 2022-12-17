use day04::parse_pair;

fn fully_contained(assignment1: (u32, u32), assignment2: (u32, u32)) -> bool{
    let r2_in_r1 = |r1: (u32, u32), r2: (u32, u32)|{
        if r1.0 <= r2.0 && r2.1 <= r1.1{
            // range 2 is fully contained in range 1
            return true;
        }
        return false;
    };
    
    if r2_in_r1(assignment1, assignment2) || r2_in_r1(assignment2, assignment1){
        return true;
    }
    return false;
}

fn main(){    
    let overlaps = include_str!("../../input.txt")
        .lines()
        .fold(0, |acc, line|{
            let assignment_pair = parse_pair(line);
            if fully_contained(assignment_pair.0, assignment_pair.1){
                return acc + 1;
            }
            return acc;
        });
    println!("{overlaps}");
     
}