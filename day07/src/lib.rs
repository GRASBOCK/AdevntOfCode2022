use std::{collections::HashMap};

pub struct FileSystem{
    pub dirs: Vec<Directory>
}

impl FileSystem{
    pub fn root(&self) -> &Directory{
        &self.dirs[0]
    }

    pub fn mkdir(&mut self, name: &str, parent_index: usize) -> &Directory{
        let new_dir_index = self.dirs.len();
        self.dirs.push(Directory::new(new_dir_index, parent_index));
        self.dirs[parent_index].dirs.insert(String::from(name), new_dir_index);
        &self.dirs[new_dir_index]
    }
}

pub struct Directory{
    parent_index: usize,
    index: usize,
    dirs: HashMap<String, usize>,
    files: HashMap<String, usize>,
}

impl Directory{
    pub fn new(index:usize, parent_index: usize) -> Self {
        Directory{index, parent_index, dirs: HashMap::new(), files: HashMap::new()}
    }

    pub fn parent<'a>(&self, fs: &'a FileSystem) -> &'a Directory{
        &fs.dirs[self.parent_index]
    }

    pub fn size(&self, fs: &FileSystem) -> usize{
        let files_size = self.files
            .iter()
            .fold(0, |acc, file|{
                let file_size = file.1;
                acc + file_size
            });
        let dirs_size = self.dirs
            .iter()
            .fold(0, |acc, dir|{
                let dir_index = *dir.1;
                let dir_size = fs.dirs[dir_index].size(fs);
                acc + dir_size
            });
        dirs_size + files_size
    }
}

fn handle_cd(fs: & FileSystem, cwd: & Directory, target: &str) -> usize{
    match target{
        "/" => fs.dirs[0].index,
        ".." => cwd.parent(&fs).index,
        _ => {
            let dir_index = *cwd.dirs.get(target).expect(format!("directory {target} doesn't exist").as_str());
            fs.dirs[dir_index].index
        }
    }
}

pub fn parse_input() -> FileSystem{
    let mut fs = FileSystem{dirs: vec![Directory{index: 0, parent_index: 0, dirs: HashMap::new(), files: HashMap::new()}]};
    let mut cwd_index = 0;
    for line in include_str!("../input.txt").lines(){
        let args = line.split_whitespace().collect::<Vec<&str>>();
        if args[0] == "$"{
            match args[1]{
                "cd" => {cwd_index = handle_cd(&fs, &fs.dirs[cwd_index], args[2]);},
                "ls" => {},
                _ => panic!("empty command line")
            }
        }else{
            match args[0]{
                "dir" => { fs.mkdir(args[1], cwd_index);},
                _ => { 
                    let filename = String::from(args[1]);
                    let filesize = args[0].parse::<usize>().unwrap();
                    fs.dirs[cwd_index].files.insert(filename, filesize);
                }
            }
        }
        
    }
    return fs;
}