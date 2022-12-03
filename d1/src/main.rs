fn main() {
    let str = include_str!("data");
    println!(
        "ANSWER: {}",
        str.split("\n\n")
            .map(|elf| elf
                .lines()
                .map(|cal| cal.trim().parse::<usize>().unwrap_or(0))
                .sum::<usize>())
            .max()
            .unwrap()
    );
    let mut x = str
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.trim().parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    x.sort();
    x.reverse();
    println!("ANSWER: {}", x[..3].into_iter().sum::<usize>());
}
