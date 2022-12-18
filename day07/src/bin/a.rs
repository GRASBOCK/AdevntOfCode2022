use day07::parse_input;

fn main(){
    let fs = parse_input();
    let size = fs.dirs
        .iter()
        .fold(0, |acc, dir|{
            let dir_size = dir.size(&fs);
            if dir_size <= 100000{
                acc + dir_size
            }else{
                acc
            }
            
        });
    println!("{size}");
}