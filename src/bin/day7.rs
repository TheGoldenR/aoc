use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

struct Filesystem {
    filesystem: Vec<Dir>,
}

impl FromIterator<Dir> for Filesystem {
    fn from_iter<T: IntoIterator<Item = Dir>>(iter: T) -> Self {
        let mut filesystem = Vec::new();

        for i in iter {
            filesystem.push(i);
        }
        return Filesystem { filesystem };
    }
}

impl Filesystem {
    fn dir_size(&self, id: &String) -> usize {
        //BUG : repeated ids
        let dir = self.filesystem.iter().find(|x| &x.id == id);
        let dir: &Dir = match dir {
            Some(x) => x,
            None => return 999999,
        };

        //adds files size
        let mut total = dir.files.iter().sum();

        // find inner_dirs
        let inner_dirs = self
            .filesystem
            .iter()
            //BUG : repeated ids
            .filter(|x| dir.dirs.contains(&x.id.to_string()));

        for i in inner_dirs {
            total += self.dir_size(&i.id);
        }

        return total;
    }

    fn all_sizes(&self) -> Vec<usize> {
        return self
            .filesystem
            .iter()
            .map(|x| self.dir_size(&x.id))
            .collect::<Vec<usize>>();
    }

    fn get_sum_eq_below(&self, val: usize) -> usize {
        return self.all_sizes().iter().filter(|x| **x <= val).sum();
    }
}

#[derive(Debug)]
struct Dir {
    id: String,
    files: Vec<usize>,
    dirs: Vec<String>,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // the input has more than 1 dir with the same name
        let mut id: &str = "default";
        let mut files: Vec<usize> = Vec::new();
        let mut dirs: Vec<String> = Vec::new();

        let mut first = true;
        //for line in s.trim().lines().skip(1) {
        for line in s.trim().lines() {
            //"/"
            //$ ls dir a
            //14848514 b.txt 8504156 c.dat
            //dir d

            if line == "$ ls" {
                continue;
            }

            if first {
                //BUG : repeated ids
                id = line;
                first = false;
                continue;
            }

            let (x, y) = match line.split_once(' ') {
                Some((x, y)) => (x, y),
                None => return Err(anyhow!("Error")),
            };

            if x == "dir" {
                //BUG : repeated ids
                dirs.push(y.to_string());
                continue;
            }

            files.push(x.parse::<usize>()?);
        }
        return Ok(Dir {
            id: id.to_string(),
            files,
            dirs,
        });
    }
}

fn main() {
    let input = include_str!("../../data/day7.test");

    println!("=== Part 1 ===");
    part1(input);
    println!("=== Part 2 ===");
}

//" /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n"
//" a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n"
//" e\n$ ls\n584 i\n"
//" d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n"
fn part1(s: &str) {
    let filesystem: Filesystem = s
        .split("$ cd")
        .skip(1)
        .filter(|&x| x.trim() != "..")
        .map(|x| x.parse::<Dir>().expect("Something has gone wrong here!!"))
        .collect::<Filesystem>();

    for i in filesystem.filesystem.iter() {
        println!("{:?}", i);
    }
    //let test_dir = "a";
    //println!(
    //    "file: {}, size: {}",
    //    test_dir,
    //    filesystem.dir_size(&String::from(test_dir))
    //);
    println!("result: {}", filesystem.get_sum_eq_below(100000));
}

//fn part2(s: &str) {
//}
