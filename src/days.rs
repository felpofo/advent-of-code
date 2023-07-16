use std::fs::read_to_string;

pub fn days() -> &'static [[fn(path: &str) -> usize; 2]] {
    &[
        // Day 1
        [
            // Part 1
            |path: &str| -> usize {
                let mut a: Vec<Vec<usize>> = vec![vec![]];

                read_to_string(path).unwrap().lines().for_each(|n| {
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

                read_to_string(path).unwrap().lines().for_each(|n| {
                    let i = a.len() - 1;

                    match n {
                        "" => a.push(vec![]),
                        _ => a[i].push(n.parse().unwrap()),
                    }
                });

                let mut a: Vec<usize> = a.into_iter().map(|elf| elf.iter().sum()).collect();

                a.sort();

                a[a.len() - 3..a.len()].into_iter().sum()
            },
        ],
    ]
}
