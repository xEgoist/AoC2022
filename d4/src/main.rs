fn main() {
    let data = include_str!("data");
    println!(
        "{}",
        data.lines()
            .map(|line| {
                let both = line.split_once(',').unwrap();
                (
                    both.0.split_once('-').unwrap(),
                    both.1.split_once('-').unwrap(),
                )
            })
            .map(|((first_a, first_b), (second_a, second_b))| (
                (
                    first_a.parse::<u8>().unwrap(),
                    first_b.parse::<u8>().unwrap()
                ),
                (
                    second_a.parse::<u8>().unwrap(),
                    second_b.parse::<u8>().unwrap()
                )
            ))
            .filter(
                |(first, second)| (first.0 <= second.0 && first.1 >= second.1)
                    || (second.0 <= first.0 && second.1 >= first.1)
            )
            .count()
    );

    println!(
        "{}",
        data.lines()
            .map(|line| {
                let both = line.split_once(',').unwrap();
                (
                    both.0.split_once('-').unwrap(),
                    both.1.split_once('-').unwrap(),
                )
            })
            .map(|((first_a, first_b), (second_a, second_b))| (
                (
                    first_a.parse::<u8>().unwrap(),
                    first_b.parse::<u8>().unwrap()
                ),
                (
                    second_a.parse::<u8>().unwrap(),
                    second_b.parse::<u8>().unwrap()
                )
            ))
            .filter(
                |(first, second)| (first.0 <= second.1 && first.1 >= second.0)
                    || (second.0 <= first.1 && second.1 >= first.0)
            )
            .count()
    );
}
