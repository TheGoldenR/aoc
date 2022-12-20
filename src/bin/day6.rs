//Test results (7,5,6,10,11)
fn main() {
    let input = include_str!("../../data/day6.prod");

    println!("=== Part 1 ===");
    let sol = solve(input, 4);
    println!("{}", sol);

    println!("=== Part 2 ===");
    let sol = solve(input, 14);
    println!("{}", sol);
}

// NotB3NNY, from Prime discord, byte approach
// Veeery clever
fn solve(i: &str, num: usize) -> usize {
    let i = i.as_bytes();
    let mut filter = 0u32;

    i.iter()
        .take(num - 1)
        .for_each(|c| filter ^= 1 << (c - b'a')); //a => 97 => 0110 0001

    i.windows(num)
        .position(|w| {
            let first = w[0];
            let last = w[w.len() - 1];
            filter ^= 1 << (last - b'a');
            let res = filter.count_ones() == num as _;
            filter ^= 1 << (first - b'a');
            res
        })
        .map(|x| x + num)
        .unwrap()
}
