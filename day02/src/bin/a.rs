fn main() {
    // rock, paper, scissor
    let shapes = [(1, 1), (2, 2), (3, 0)]; // (shape_reward, counter shape index)
    let score = |opponent: usize, me: usize| -> u32{
        let (_opponent_shape_reward, opponent_counter) = shapes[opponent];
        let (me_shape_reward, _me_counter) = shapes[me];
        if opponent == me{
            return me_shape_reward + 3; // draw
        }
        if opponent_counter == me{
            return me_shape_reward + 6;
        }
        return me_shape_reward;
    };

    let score = include_str!("../../input.txt")
        .lines()
        .fold(0, |acc, text|{
            let mut abc_xyz = text.split_whitespace();
            
            let opponent = abc_xyz.next().unwrap();
            let me = abc_xyz.next().unwrap();
            let opponent = match opponent{
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!("unexpected opponent shape {}", opponent)
            };
            let me = match me{
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => panic!("unexpected me shape {}", me)
            };

            acc + score(opponent, me)
        });

    println!("{}", score);
}
