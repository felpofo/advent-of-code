#![feature(exclusive_range_pattern)]

mod days;
use days::*;

fn main() {
    let days = days();

    days.iter().enumerate().for_each(|(mut i, [a, b])| {
        i += 1;

        let file = &format!("day_{i}_input.txt");

        let a = a(file);
        let b = b(file);

        println!("Day {i} :: a={a}, b={b}");
    });
}
