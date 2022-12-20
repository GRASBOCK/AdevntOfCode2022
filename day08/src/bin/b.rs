use day08::{parse_input};
use ndarray::Array2;

fn highest_scenic_score(trees: &Array2<i8>) -> usize{
    let visible_along_dir = |origin: (usize, usize), next: [i32; 2]| -> usize{
        let origin_height = trees[origin];
        let mut counter = 0;
        let mut last_index = origin;
        loop {
            let index = [last_index.0 as i32 + next[0], last_index.1 as i32 + next[1]];
            if index[0] < 0 || index[1] < 0{
                // edge
                break;
            }
            let index = [usize::try_from(index[0]).unwrap(), usize::try_from(index[1]).unwrap()];
            last_index = (index[0], index[1]);
            let tree_height = trees.get(index);
            if tree_height.is_none(){
                // edge
                break;
            }
            let tree_height = *tree_height.unwrap();
            if tree_height >= origin_height{
                counter += 1;
                break;
            }
            counter += 1;
        }
        if counter == 0{
            return 1;
        }
        return counter;
    };

    let shape = trees.shape();
    let (nrows, ncols) = (shape[0], shape[1]);
    trees
        .indexed_iter()
        .map(|(index, _height)|{
            if index.0 == 0 || index.0 == nrows-1 || index.1 == 0 || index.1 == ncols-1 {
                return 0;
            }
            let up = visible_along_dir(index, [-1, 0]);
            let down = visible_along_dir(index, [1, 0]);
            let right = visible_along_dir(index, [0, -1]);
            let left = visible_along_dir(index, [0, 1]);
            //println!("index: [{}, {}], {up} * {down} * {right} * {left}", index.0, index.1);
            up * down * left * right
        })
        .max()
        .unwrap()
}

fn main(){
    let input = include_str!("../../input.txt");
    let trees = parse_input(input);
    let highest_score = highest_scenic_score(&trees);
    println!("{highest_score}");
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, highest_scenic_score};

    #[test]
    fn highest_score() {
        let input = include_str!("../../input_sample.txt");
        let trees = parse_input(input);
        let highest_score = highest_scenic_score(&trees);
        assert_eq!(highest_score, 8);
    }
}
