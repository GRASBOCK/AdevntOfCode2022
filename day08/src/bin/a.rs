use day08::{parse_input};
use ndarray::Array2;


pub fn visible_trees_map(trees: &Array2<i8>) -> Array2<i8>{
    let mut visible = Array2::<i8>::zeros(trees.raw_dim());

    let set_visible_along_dir = |visible: &mut Array2<i8>, start: [i32; 2], next: [i32; 2], count: usize|{
        let mut blocking_tree_height = -1;
        for i in 0..count as i32{
            let index = [start[0] + i * next[0], start[1] + i * next[1]];
            let index = [usize::try_from(index[0]).unwrap(), usize::try_from(index[1]).unwrap()];
            let tree_height = trees[index];
            if tree_height > blocking_tree_height{
                blocking_tree_height = tree_height;
                visible[index] = 1;
            }
        }
    };

    let shape = trees.shape();
    let (nrows, ncols) = (shape[0], shape[1]);
    for row in 0..nrows as i32{
        let left = [row, 0];
        let next = [0, 1];
        set_visible_along_dir(&mut visible, left, next, ncols-1);
        //println!("l {row}\n{visible}\n");
        let right = [row, ncols as i32 -1];
        let next = [0, -1];
        set_visible_along_dir(&mut visible, right, next, ncols-1);
        //println!("r {row}\n{visible}\n");
    }
    for col in 0..ncols as i32 {
        let top = [0, col];
        let next = [1, 0];
        set_visible_along_dir(&mut visible, top, next, nrows-1);
        //println!("t {col}\n{visible}\n");
        let bottom = [nrows as i32 -1, col];
        let next = [-1, 0];
        set_visible_along_dir(&mut visible, bottom, next, nrows-1);
        //println!("b {col}\n{visible}\n");
    }
    visible
}

pub fn visible_tree_count(visbile: &Array2<i8>) -> usize{
    visbile
        .iter()
        .fold(0, |acc, visibility| acc+(*visibility as usize))
}

fn main(){
    let input = include_str!("../../input.txt");
    let trees = parse_input(input);
    let visbile = visible_trees_map(&trees);
    let visible_tree_count = visbile
        .iter()
        .fold(0, |acc, visibility| acc+(*visibility as usize));
    println!("{visible_tree_count}")
}


#[cfg(test)]
mod tests {
    use ndarray::arr2;

    use crate::{parse_input, visible_trees_map, visible_tree_count};

    #[test]
    fn visibility_map() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let trees = parse_input(input);
        let visbile = visible_trees_map(&trees);
        let vtc = visible_tree_count(&visbile);
        let actually_visible = arr2(&[[1,1,1,1,1],[1,1,1,0,1],[1,1,0,1,1],[1,0,1,0,1],[1,1,1,1,1]]);
        assert_eq!(visbile, actually_visible);
        assert_eq!(vtc, 21);
    }
}
