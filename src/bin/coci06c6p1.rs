use std::{collections::HashMap, io::BufRead};

/**
 * COCI '06 Contest 6 #1 Prase
 *
 * N children are eating lunch at the table. Children take turns in taking food from the table.
 *
 * Some of the children haven't yet been taught proper manners so they jump at the food without giving the others a chance.
 * If at any point a child takes a piece of food, and that child had already taken more food than the other children all
 * together (not including the new piece of food), then the mother will warn that child to behave.
 *
 * You will be given the order in which the children take food. Write a program that calculates how many times the mother
 * has to warn the children.
 *
 * Input Specification
 * The first line of input contains an integer N ( 1 ≤ N ≤ 100 ) , how many pieces of food the children take.
 *
 * Each of the following N lines contains the name of a child that took one piece of food. The names will be strings of at
 * most 20 lowercase letters of the English alphabet.
 *
 * Output Specification
 * Output the number of warnings on a single line.
 *
 * Sample Input 1
 * 4
 * mirko
 * stanko
 * stanko
 * stanko
 *
 * Sample Output 1
 * 1
 *
 * Sample Input 2
 * 17
 * a
 * b
 * b
 * a
 * a
 * a
 * c
 * a
 * b
 * b
 * c
 * b
 * b
 * b
 * b
 * b
 * b
 *
 * Sample Output 2
 * 4
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut child_taken = HashMap::new();
    let mut warnings = 0;
    for (total_taken, line) in lines.take(n).enumerate() {
        let line = line.unwrap();
        let child = child_taken.entry(line).or_insert(0usize);

        if 2 * *child > total_taken {
            warnings += 1;
        }

        *child += 1;
    }

    println!("{warnings}");
}
