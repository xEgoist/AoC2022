#[derive(PartialEq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
enum WLD {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

fn main() {
    let data = include_str!("data");
    let res: usize = data
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            let mfirst = match first {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!("WTF"),
            };
            let msecond = match second {
                "X" => RPS::Rock,
                "Y" => RPS::Paper,
                "Z" => RPS::Scissors,
                _ => panic!("WTF"),
            };
            (mfirst, msecond)
        })
        .map(|(a, b)| {
            if a == b {
                b as usize + 3
            } else if (a == RPS::Rock && b == RPS::Paper)
                || (a == RPS::Paper && b == RPS::Scissors)
                || (a == RPS::Scissors && b == RPS::Rock)
            {
                b as usize + 6
            } else {
                b as usize
            }
        })
        .sum();

    let res2: usize = data
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            let mfirst = match first {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!("WTF"),
            };
            let msecond = match second {
                "X" => WLD::Loss,
                "Y" => WLD::Draw,
                "Z" => WLD::Win,
                _ => panic!("WTF"),
            };
            (mfirst, msecond)
        })
        .map(|(a, b)| match (&a, b) {
            (_, WLD::Win) => {
                if a == RPS::Rock {
                    6 + RPS::Paper as usize
                } else if a == RPS::Scissors {
                    6 + RPS::Rock as usize
                } else {
                    6 + RPS::Scissors as usize
                }
            }
            (_, WLD::Loss) => {
                if a == RPS::Rock {
                    RPS::Scissors as usize
                } else if a == RPS::Scissors {
                    RPS::Paper as usize
                } else {
                    RPS::Rock as usize
                }
            }
            (_, WLD::Draw) => 3 + a as usize,
        })
        .sum();
    println!("{res}");
    println!("{res2}");
}
