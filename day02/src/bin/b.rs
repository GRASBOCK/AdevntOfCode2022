fn main() {
    // rock, paper, scissor
    let shapes = [(1, 2, 1), (2, 0, 2), (3, 1, 0)]; // (shape_reward, beat shape index, counter shape index)
    let score = |opponent: usize, me: usize| -> u32{
        let opponent_counter = shapes[opponent].2;
        let me_shape_reward = shapes[me].0;
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
            let opponent_counter = shapes[opponent].2;
            let opponent_beat = shapes[opponent].1;
            let me = match me{
                "X" => opponent_beat,
                "Y" => opponent,
                "Z" => opponent_counter,
                _ => panic!("unexpected me shape {}", me)
            };

            acc + score(opponent, me)
        });

    println!("{}", score);
}
