use std::str::FromStr;
use anyhow::Result;

//A,X rock - 1
//B,Y paper - 2
//C,Z scissor - 3
// === PART 2
// X LOOSE
// Y DRAW
// Z WIN

// rock > scissor
// paper > rock
// scissor > paper

// Win  6
// Draw 3
// Loss 0
//

//    | 3 | 2 | 1
//    | A | B | C
//1 X | 3 | 0 | 6
//2 Y | 6 | 3 | 0
//3 Z | 0 | 6 | 3
//
//    | 3 | 2 | 1
//  1 | 4 | 3 | 2
//  2 | 5 | 4 | 3
//  3 | 6 | 5 | 4

#[derive(Debug)]
struct Round {
   score: usize,
   score2: usize 
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(round: &str) -> Result<Self> {

        let (a,x) = match round.split_once(' ') {
            Some((a,x)) => (a,x),
            None => return Err(anyhow::anyhow!("This input sucks")),
        };
        
        let elf_play = to_number(&a);
        let my_play = to_number(&x);
        let score = round_result(elf_play, my_play);
        //part2
        let score2 = play_mappping(&x, &a);

        return Ok(Round { score, score2 });
    }
}

fn to_number(play: &str) -> usize {
    return match play {
        "X" | "C" => 1,
        "Y" | "B" => 2,
        "Z" | "A" => 3,
        _ => 100
    }
}

fn round_result(elf: usize, me: usize) -> usize {
    let fight = elf + me;

    return match fight {
        4 => 3 + me, //draw
        2 | 5 => 6 + me, //win
        _ => 0 + me //loose
    } 
}

fn play_mappping(me: &str, elf: &str) -> usize {
    let elf_value =  match elf {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 8888
    };
    
    return match me {
        "X" => if elf_value - 1 < 1 { 3 } else { elf_value - 1 }, //loose
        "Y" => elf_value + 3, //draw
        "Z" =>  if elf_value + 1 > 3 { 7 } else { elf_value + 7 }, //win
        _ => 9999
    }

}

pub fn challenge(){

    let game = include_str!("../data/day2.prod").lines()
        .flat_map(|x| x.parse()).collect::<Vec<Round>>();

    //println!("{:?}", &game);
    println!("{:?}", &game.iter().map(|x| x.score).sum::<usize>());
    println!("{:?}", &game.iter().map(|x| x.score2).sum::<usize>());

}

