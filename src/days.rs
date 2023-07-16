use std::fs::read_to_string;

pub fn days() -> &'static [[fn(path: &str) -> usize; 2]] {
    &[
        // Day 1
        [
            // Part 1
            |path: &str| -> usize {
                let mut a: Vec<Vec<usize>> = vec![vec![]];

                read_to_string(path)
                    .unwrap()
                    .trim_end()
                    .lines()
                    .for_each(|n| {
                        let i = a.len() - 1;

                        match n {
                            "" => a.push(vec![]),
                            _ => a[i].push(n.parse().unwrap()),
                        }
                    });

                a.into_iter().map(|elf| elf.iter().sum()).max().unwrap()
            },
            // Part 2
            |path: &str| -> usize {
                let mut a: Vec<Vec<usize>> = vec![vec![]];

                read_to_string(path)
                    .unwrap()
                    .trim_end()
                    .lines()
                    .for_each(|n| {
                        let i = a.len() - 1;

                        match n {
                            "" => a.push(vec![]),
                            _ => a[i].push(n.parse().unwrap()),
                        }
                    });

                let mut a: Vec<usize> = a.into_iter().map(|elf| elf.iter().sum()).collect();

                a.sort();

                a[a.len() - 3..a.len()].into_iter().sum()
            }
        ],
        // Day 2
        [
            |path: &str| -> usize {
                let mut score = 0;

                const ROCK: usize = 1;
                const PAPER: usize = 2;
                const SCISORS: usize = 3;

                const LOSE: usize = 0;
                const DRAW: usize = 3;
                const WIN: usize = 6;

                read_to_string(path)
                    .unwrap()
                    .trim_end()
                    .lines()
                    .map(|game| game.split(' ').collect::<Vec<_>>())
                    .map(|x| (x[0].parse::<char>().unwrap(), x[1].parse::<char>().unwrap()))
                    .for_each(|(a, b)| match a {
                        // Rock
                        'A' => match b {
                            'X' => score += ROCK + DRAW,
                            'Y' => score += PAPER + WIN,
                            'Z' => score += SCISORS + LOSE,
                            _ => unreachable!(),
                        },
                        // Paper
                        'B' => match b {
                            'X' => score += ROCK + LOSE,
                            'Y' => score += PAPER + DRAW,
                            'Z' => score += SCISORS + WIN,
                            _ => unreachable!(),
                        },
                        // Scisors
                        'C' => match b {
                            'X' => score += ROCK + WIN,
                            'Y' => score += PAPER + LOSE,
                            'Z' => score += SCISORS + DRAW,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    });

                score
            },
            |path: &str| -> usize {
                let mut score = 0;

                const ROCK: usize = 1;
                const PAPER: usize = 2;
                const SCISORS: usize = 3;

                const LOSE: usize = 0;
                const DRAW: usize = 3;
                const WIN: usize = 6;

                read_to_string(path)
                    .unwrap()
                    .trim_end()
                    .lines()
                    .map(|game| game.split(' ').collect::<Vec<_>>())
                    .map(|x| (x[0].parse::<char>().unwrap(), x[1].parse::<char>().unwrap()))
                    .for_each(|(a, b)| match a {
                        // Rock
                        'A' => match b {
                            'X' => score += SCISORS + LOSE,
                            'Y' => score += ROCK + DRAW,
                            'Z' => score += PAPER + WIN,
                            _ => unreachable!(),
                        },
                        // Paper
                        'B' => match b {
                            'X' => score += ROCK + LOSE,
                            'Y' => score += PAPER + DRAW,
                            'Z' => score += SCISORS + WIN,
                            _ => unreachable!(),
                        },
                        // Scisors
                        'C' => match b {
                            'X' => score += PAPER + LOSE,
                            'Y' => score += SCISORS + DRAW,
                            'Z' => score += ROCK + WIN,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    });

                score
            }
        ],
        // Day 3
        [
            // Part 1
            |path: &str| -> usize {
                read_to_string(path)
                    .unwrap()
                    .trim_end()
                    .lines()
                    .filter_map(|b| {
                        let half = b.len() / 2;

                        let compartiments = (&b[..half], &b[half..]);

                        for c in compartiments.1.bytes() {
                            let a = compartiments.0.bytes().find(|&a| a == c);
                            if let Some(a) = a {
                                return Some(char::from(a));
                            }
                        }

                        None
                    }).map(|c| match c {
                        'a'..='z' => c as usize - 96,
                        'A'..='Z' => c as usize - 64 + 26,
                        _ => unreachable!(),
                    }).sum()
            },
            // Part 2
            |path: &str| -> usize {
                read_to_string(path)
                    .unwrap()
                    .trim_end()
                    .lines()
                    .collect::<Vec<_>>()
                    .chunks_exact(3)
                    .map(|g| {
                        for a in g[0].bytes() {
                            let b = g[1].bytes().find(|&b| b == a);
                            let c = g[2].bytes().find(|&c| c == a);

                            if b.is_some() && c.is_some() {
                                return match char::from(a) {
                                    'a'..='z' => a as usize - 96,
                                    'A'..='Z' => a as usize - 64 + 26,
                                    _ => unreachable!(),
                                }
                            }
                        }

                        unreachable!()
                    }).sum()
            }
        ],
        // // Day X
        // [
        //     // Part 1
        //     |path: &str| -> usize {},
        //     // Part 2
        //     |path: &str| -> usize {}
        // ],
    ]
}
