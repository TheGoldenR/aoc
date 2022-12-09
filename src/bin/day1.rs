fn main() {
    let mut sorted_calories = include_str!("../../data/day1.prod").split("\n\n")
        .map(|x| {
            return x.split('\n').flat_map(str::parse::<usize>)
                .sum::<usize>();
        }).collect::<Vec<usize>>();

    println!("Part 1 => {:?}", sorted_calories.iter().max().unwrap());

    sorted_calories.sort_by(|a, b| b.cmp(a));

    println!("Part 2 => {:?}\n", sorted_calories.iter().take(3).sum::<usize>());
}

