use std::collections::{ HashSet, HashMap };

fn item_map(x: char) -> u32{
    let digit = x as u32;

    // lowercase 1-26 (ascii [97,122]) = -96, uppercase 27-52 (ascii [65, 90]) = -38
    return if digit > 91 { digit - 96 } else { digit - 38 };
}

fn main(){

    let rucksacks = include_str!("../../data/day3.prod").lines();

    let all_priorities = rucksacks.map(|x| {
        let (comp1, comp2) = x.split_at(x.len()/2);

        let priorities = comp2.chars()
            .filter(|x| 
                    comp1.chars().any(|y| y==*x))
            .map(|x| item_map(x))
            .collect::<HashSet<_>>();

        return priorities.iter().sum::<u32>();
    });
    
    println!("=== Part 1 ===");
    println!("{}", all_priorities.sum::<u32>());

    println!("=== Part 2 ===");

    let groups: Vec<&str> = include_str!("../../data/day3.prod").lines().collect();

    let x = groups.chunks(3).map(|group| { 
        return group.iter()
            .flat_map(|line| { 
                return line.chars().collect::<HashSet<_>>();

            })
        .fold(HashMap::new(), |mut map: HashMap<char, u32>, badge| {
            map.entry(badge).and_modify(|counter| *counter += 1).or_insert(1);

            return map;

        })
        .into_iter()
            .filter(|(_k, value)| *value==3)
            .collect::<HashMap<char,u32>>()
            .into_keys().collect::<Vec<char>>();

    }).map(|x| item_map(x[0]))
    .sum::<u32>();

    println!("{}", x);
}
