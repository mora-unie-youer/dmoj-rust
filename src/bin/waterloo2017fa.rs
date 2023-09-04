use std::io::BufRead;

/**
 * Waterloo 2017 Fall A - Art
 * 2017 Fall Waterloo Local ACM Contest, Problem A
 *
 * Vera has five sticks of distinct lengths l 1 , l 2 , l 3 , l 4 , l 5 . Vera may choose any three of the five sticks to
 * form the sides of a triangle. How many different triangles can Vera make? Each triangle must have positive area and
 * sticks cannot be bent or cut.
 *
 * Input
 * Line 1 contains integers l 1 , l 2 , l 3 , l 4 , l 5 ( 1 ≤ l i ≤ 1000 ) .
 *
 * Output
 * Print one line with one integer, the number of ways to form a triangle.
 *
 * Sample Input 1
 * 1 2 3 4 5
 *
 * Sample Output 1
 * 3
 *
 * Sample Input 2
 * 1 2 4 8 16
 *
 * Sample Output 2
 * 0
 *
 * Note
 * For the first example, the 3 ways to form a triangle are choosing sticks 2 , 3 , 4 or 2 , 4 , 5 or 3 , 4 , 5 .
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut ls: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .take(5)
        .map(|v| v.parse().unwrap())
        .collect();
    ls.sort_unstable();

    let mut count = 0;
    for i in 0..ls.len() {
        let a = ls[i];
        for j in i + 1..ls.len() {
            let b = ls[j];
            for &c in ls.iter().skip(j + 1) {
                if a + b > c {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}
