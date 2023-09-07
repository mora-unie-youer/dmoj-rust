use std::{collections::HashSet, io::BufRead};

/**
 * Unique Elements
 *
 * Given a list of N ( 1 ≤ N ≤ 30 000 ) positive integers less than or equal to 10 9 , print out how many distinct numbers exist.
 *
 * Input Specification
 * The first line will contain the integer N . The next N lines will contain an element in the list.
 *
 * Output Specification
 * One integer: the number of distinct elements in the given list.
 *
 * Sample Input 1
 * 2
 * 1
 * 2
 *
 * Sample Output 1
 * 2
 *
 * Sample Input 2
 * 4
 * 1
 * 2
 * 2
 * 5
 *
 * Sample Output 2
 * 3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let elements: HashSet<usize> = lines
        .take(n)
        .map(Result::unwrap)
        .map(|line| line.parse().unwrap())
        .collect();
    println!("{}", elements.len());
}
