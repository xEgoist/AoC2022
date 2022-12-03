use std::collections::HashSet;
fn main() {
    let data = include_str!("data");
    let res1: usize = data
        .lines()
        .map(|line| line.as_bytes().split_at(line.len() / 2))
        .map(|(first, second)| {
            let set_a: HashSet<_> = first.iter().collect();
            let set_b: HashSet<_> = second.iter().collect();
            let mut v = 0;
            for byte in set_a {
                if set_b.contains(byte) {
                    v += (((byte % 123) - 38) % 58) as usize;
                }
            }
            v
        })
        .sum();

    let res2: usize = data
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            let mut v = 0;
            let set_b = lines[1].as_bytes().iter().collect::<HashSet<_>>();
            let set_c = lines[2].as_bytes().iter().collect::<HashSet<_>>();
            for byte in lines[0].as_bytes().iter().collect::<HashSet<_>>() {
                if set_b.contains(byte) && set_c.contains(byte) {
                    v += (((byte % 123) - 38) % 58) as usize;
                }
            }
            v
        })
        .sum();
    println!("{res1}");
    println!("{res2}");
}
