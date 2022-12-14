use anyhow:: { Result, Error, anyhow };
use std::str::FromStr;

struct Crane { 
    supplies: Vec<Vec<char>> ,
    moves: Vec<Move>,
}

impl FromStr for Crane {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

       let (supplies, moves) = match s.split_once("\n\n") {
           Some((x,y)) => (x,y),
           None => return Err(anyhow!("oh no"))
       };

       //Supplies
       let supplies = get_stacks(supplies);
       
       //Moves 
       let moves = moves.lines()
           .filter_map(|x| x.parse::<Move>().ok())
           .collect::<Vec<Move>>();

       return Ok(Crane { supplies, moves });
    }
}

fn get_stacks(s: &str) -> Vec<Vec<char>> {

    let mut supplies_list= s.lines().rev().collect::<Vec<&str>>();
    let stack_idx = supplies_list.remove(0).chars().count()/4 + 1;
    let mut supplies: Vec<Vec<char>> = vec![vec![];stack_idx];

    supplies_list.iter().for_each(|line| {

        let list: Vec<char> = line.chars().collect();
        let mut stack_id = 0;
        let mut col_count = 0; 

        while col_count < list.len() {

            if list[col_count] == '[' { supplies[stack_id].push(list[col_count + 1]) }
            col_count += 4;
            stack_id += 1;
        }

    });

    return supplies;
}

impl Crane { 
    //Part1
    fn push_pop(&mut self) {
        //pop
        for mv in self.moves.iter() { 
            let mut x = mv.times;

            while x > 0 {
                let transaction = self.supplies[mv.from-1].pop().expect("this is useless here but ok");
                self.supplies[mv.to-1].push(transaction);

                x-=1;
            }
        }
    }

    //Part 2
    fn pop_multiple(&mut self) { 
        
        for mv in self.moves.iter() { 

            let mut temp: Vec<char> = Vec::new();
            let mut times = mv.times;

            while times > 0 {

                let transaction = self.supplies[mv.from-1].pop().expect("this is useless here but ok");
                temp.push(transaction);

                times -= 1;
            }

            while !temp.is_empty() {
                self.supplies[mv.to-1].push(temp.pop().expect("sd"));
            }
        }
    }

    // Joins the top element from each stack
    fn last_join(&self) { 
        let mut str = String::new();

        for sup in &self.supplies {
            str.push(*sup.last().unwrap());
        }
        println!("{}", str);
    }
}

struct Move { 
    times: usize, 
    from: usize, 
    to: usize,
}

impl FromStr for Move { 
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        return Ok(s.split_whitespace()
          .filter_map(|x| x.parse::<usize>().ok())
          .collect::<Move>()
          );
    }
}

impl FromIterator<usize> for Move { 

    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let iter = iter.into_iter().collect::<Vec<usize>>();

        return Move { times: iter[0], from: iter[1], to: iter[2] };

    }
}

fn main() {

    let input = include_str!("../../data/day5.prod");

    println!("=== Part1 ===");
    part1(input);

    println!("=== Part2 ===");
    part2(input);
}

fn part1(input: &str) {

   let mut crane = input.parse::<Crane>().unwrap();
   crane.push_pop();
   crane.last_join();
}

fn part2(input: &str) {

   let mut crane = input.parse::<Crane>().unwrap();

   crane.pop_multiple();
   crane.last_join();
}
