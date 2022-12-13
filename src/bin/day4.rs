use std::str::FromStr;
use anyhow::Result;

struct Section { 
    start: u8,
    end: u8
}

impl FromStr for Section { 
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = match s.split_once('-') { 
            Some((x,y)) => (x,y),
            None => return Err(anyhow::anyhow!("This input sucks")),
        };

        return Ok( Section { 
            start: start.parse::<u8>()?,
            end: end.parse::<u8>()? 
        }) ;
    }
}

struct Pair { 
    s1: Section,
    s2: Section
}

impl FromStr for Pair { 
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (s1, s2) = match s.split_once(',') { 
            Some((x,y)) => (x,y),
            None => return Err(anyhow::anyhow!("This input sucks")),
        };

        return Ok(Pair {
            s1: s1.parse()?,
            s2: s2.parse()?
            });
    }
}

impl Pair { 
    fn full_contains(&self) -> bool {
        return self.s1.start <= self.s2.start && self.s1.end >= self.s2.end 
            || self.s1.start >= self.s2.start && self.s1.end <= self.s2.end;
    }

    fn overlaps(&self) -> bool { 
        return self.s1.start >= self.s2.start && self.s1.start <= self.s2.end 
            || self.s2.start >= self.s1.start && self.s2.start <= self.s1.end
            || self.s1.end <= self.s2.end && self.s1.end >= self.s2.start
            || self.s2.end <= self.s1.end && self.s2.end >= self.s1.start;
    }
}

fn main() {
    let input = include_str!("../../data/day4.prod");

    println!("=== Part 1 ===");
    part1(input);
    println!("=== Part 2 ===");
    part2(input);
}

fn part1(input: &str){

    let count = input.lines()
        .map(|x|
             x.parse::<Pair>().expect("This is wrong wrong wrong").full_contains()
            ).filter(|x| *x).count();

    println!("{:?}", count);
}

fn part2(input: &str){
    let count = input.lines()
        .map(|x|
             x.parse::<Pair>().expect("This is wrong wrong wrong").overlaps()
            ).filter(|x| *x).count();

    println!("{:?}", count);
}
