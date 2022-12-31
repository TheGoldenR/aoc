use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

#[derive(Debug)]
struct Dir {
    name: String,
    size: usize,
    inner_dirs: Vec<String>,
}

impl FromStr for Dir {
    type Err = Error;

    //" /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n"
    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.trim().lines().into_iter();
        let dir_name = s.next().unwrap().to_string();
        let mut size = 0;
        let mut inner_dirs: Vec<String> = Vec::new();

        for line in s {
            if line == "$ ls" {
                continue;
            }

            let (x, y) = match line.split_once(' ') {
                Some((x, y)) => (x, y),
                None => return Err(anyhow!("yikes")),
            };

            if x == "dir" {
                inner_dirs.push(y.to_string());
                continue;
            }

            size += x.parse::<usize>()?;
        }

        return Ok(Dir {
            name: dir_name,
            size,
            inner_dirs,
        });
    }
}

#[derive(Debug)]
struct Filesystem {
    //files: Vec<(String, usize)>,
    files: Vec<Dir>,
}

impl FromIterator<Dir> for Filesystem {
    fn from_iter<T: IntoIterator<Item = Dir>>(iter: T) -> Self {
        let mut filesystem: Vec<Dir> = Vec::new();

        for i in iter {
            filesystem.push(i);
        }

        return Filesystem { files: filesystem };
    }
}

fn dir_size(filesystem: &mut Filesystem, dir: String, vec: &mut Vec<(String, usize)>) -> usize {
    let dir = filesystem.files.remove(
        filesystem
            .files
            .iter()
            .position(|x| x.name == dir)
            .expect("ahh shit here we go again"),
    );

    let mut file_size = dir.size;

    for i in dir.inner_dirs {
        file_size += dir_size(filesystem, i, vec);
    }

    vec.push((dir.name, file_size));

    return file_size;
}

fn main() {
    let input = include_str!("../../data/day7.prod");

    println!("=== Part 1 ===");
    part1(&input);
    println!("=== Part 2 ===");
}

// ('/', 4332453, )

//" /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n"
//" a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n"
//" e\n$ ls\n584 i\n"
//" d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n"
fn part1(s: &str) {
    let mut filesystem = s
        .split("$ cd")
        .skip(1)
        .filter(|x| x.trim() != "..")
        .map(|x| x.parse::<Dir>().expect("Some error"))
        .collect::<Filesystem>();

    //    println!("{:?}", filesystem.files);
    let mut vec: Vec<(String, usize)> = Vec::new();
    dir_size(&mut filesystem, "/".to_string(), &mut vec);

    println!(
        "{:?} ",
        vec.iter()
            .filter(|x| x.1 < 100000)
            .map(|x| x.1)
            .sum::<usize>()
    );

    for i in vec.into_iter() {
        println!("file: {:?}, size: {:?}", i.0, i.1);
    }
}

//fn part2(s: &str) {
//}
