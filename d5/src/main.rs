fn main() {
    let data = include_str!("data");
    let stack: Vec<&str> = data.split("\n\n").collect();   
    let mut stacks: Vec<Vec<char>> = stack.first().unwrap().lines().last().unwrap().chars().collect::<Vec<char>>()
        .iter()
        .filter(|c|c.is_ascii_digit()).map(|_| vec![]).collect();
    println!("stacks: {}",stacks.len());
    for line in stack.first().unwrap().lines() {
        for (i, ch) in line.chars().collect::<Vec<char>>().iter().skip(1).step_by(2).enumerate() {
            if ch.is_alphabetic() {
                stacks[i/2].push(*ch);
            }
            println!("PROCESSING: {:?}", ch);
        }
    }
    //for part2
    stacks.iter_mut().for_each(|x| x.reverse());
    let mut stacks2 = stacks.clone();
    println!("stacks: {:?}",stacks);
    for instruction in stack.last().unwrap().lines() {
        let xx: Vec<usize> = instruction.split_whitespace().filter_map(|num| num.trim().parse::<usize>().ok()).collect();
        for _ in 0..xx[0] {
            let s = stacks[xx[1] as usize - 1].pop().unwrap();
            stacks[xx[2] as usize - 1].push(s);
        }

    }
    stacks.iter().map(|x| x.last().unwrap()).for_each(|x| print!("{}",x));
    println!();


    // part 2
    
    for instruction in stack.last().unwrap().lines() {
        let xx: Vec<usize> = instruction.split_whitespace().filter_map(|num| num.trim().parse::<usize>().ok()).collect();
            let len = stacks2[xx[1] as usize - 1].len();
            let mut s: Vec<char> = stacks2[xx[1] as usize - 1].drain(len - xx[0]..).collect();
            stacks2[xx[2] as usize - 1].append(&mut s);

    }
    stacks2.iter().map(|x| x.last().unwrap()).for_each(|x| print!("{}",x));
    println!()
}
