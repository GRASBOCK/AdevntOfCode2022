use day07::parse_input;

fn main(){
    let fs = parse_input();
    let total_space = 70000000;
    let update_requirement = 30000000;
    
    let mut dirs = fs.dirs
        .iter()
        .map(|dir|{
            dir.size(&fs) // optimization potential: reuse size calculation of child directories
        })
        .collect::<Vec<usize>>();
    let root_size = dirs[0];
    let free_space = total_space - root_size;
    let minimum_size_to_be_deleted = update_requirement - free_space;
    dirs.sort_unstable();
    let mut remove_dir_size = 0;
    for size in dirs{
        if size >= minimum_size_to_be_deleted{
            remove_dir_size = size;
            break;
        }
    }
    
    println!("{remove_dir_size}");
}