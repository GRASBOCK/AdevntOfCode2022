use ndarray::Array2;

fn parse_row(line: &str) -> Vec<i8>{
    line
    .chars()
    .map(|char| {
        let val_u32 = char
        .to_digit(10)
        .expect(format!("char \'{char}\' is not a digit").as_str())
        as i8;
        i8::from(val_u32)
    })
    .collect()
}

pub fn parse_input(input: &str) -> Array2<i8>{
    let (ncols, grid) = {
        let mut grid = Vec::<i8>::new();
        let mut lines = input.lines();
        let ncols = {
            let row = parse_row(lines.next().unwrap());
            grid.extend(row.iter());
            grid.len()
        };
        for line in lines{
            let row = parse_row(line);
            grid.extend(row.iter());
        }
        (ncols, grid)
    };
    let nrows = grid.len()/ncols;
    let arr = Array2::from_shape_vec((nrows, ncols), grid).unwrap();
    return arr;
}
