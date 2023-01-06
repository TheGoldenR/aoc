fn edge_trees(table: &Vec<Vec<usize>>) -> usize {
    table.len() * 2 + table[0].len() * 2 - 4
}

fn row_check(table: &Vec<Vec<usize>>) -> usize {
    let last_el = table[0].len() - 1;
    let visible_trees = 0;

    for i in 0..table.len() {
        for i in 1..last_el - 1 {
            let first = table[i][0];
            let last = table[i][last_el];
            //
        }
    }

    visible_trees
}

fn main() {
    let input = include_str!("../../data/day8.test")
        .lines()
        .map(|row| row.chars().map(|el| el as usize).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    println!("=== Part 1 ===");
    part1(&input);

    println!("=== Part 2 ===");
    part2(&input);
}

fn part1(table: &Vec<Vec<usize>>) {
    let edge_trees: usize = edge_trees(&table);
}

fn part2(table: &Vec<Vec<usize>>) {
    //
}
