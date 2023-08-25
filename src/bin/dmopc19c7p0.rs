use std::io::BufRead;

/**
 * DMOPC '19 Contest 7 P0 - Contest Feedback
 *
 * A certain contest with 4 problems has just finished running! As one of the problem setters, Mimi is interested in knowing what contestants thought of the problems.
 *
 * After surveying everyone, she realizes that people rated the first problem an average of A out of 10, the second problem B out of 10, the third C out of 10, and the fourth D out of 10.
 *
 * Mimi then wants to know:
 *   What was the average rating of problems 1 and 2?
 *   What was the average rating of problems 1 and 3?
 *   What was the average rating of problems 1 and 4?
 *   What was the average rating of problems 2 and 3?
 *   What was the average rating of problems 2 and 4?
 *   What was the average rating of problems 3 and 4?
 *   What was the average rating of problems 1, 2, and 3?
 *   What was the average rating of problems 1, 2, and 4?
 *   What was the average rating of problems 1, 3, and 4?
 *   What was the average rating of problems 2, 3, and 4?
 *   What was the average rating of all the problems?
 * As Mimi is busy setting next year's contest, this task falls to you. Can you answer Mimi's queries?
 *
 * Input Specification
 *
 * The input will consist of 4 space separated integers, A , B , C , D , respectively.
 *
 * These integers satisfy 0 ≤ A , B , C , D ≤ 10 .
 *
 * Output Specification
 *
 * The answer to each query, each on a separate line.
 *
 * Your answer will be judged as correct if it is within an absolute error of 10^−6 .
 *
 * Sample Input
 * 1 2 3 4
 *
 * Sample Output
 * 1.500000
 * 2.000000
 * 2.500000
 * 2.500000
 * 3.000000
 * 3.500000
 * 2.000000
 * 2.333333
 * 2.666667
 * 3.000000
 * 2.500000
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap());
    let a: f64 = first_line.next().unwrap();
    let b = first_line.next().unwrap();
    let c = first_line.next().unwrap();
    let d = first_line.next().unwrap();

    println!("{:.6}", (a + b) / 2.);
    println!("{:.6}", (a + c) / 2.);
    println!("{:.6}", (a + d) / 2.);
    println!("{:.6}", (b + c) / 2.);
    println!("{:.6}", (b + d) / 2.);
    println!("{:.6}", (c + d) / 2.);
    println!("{:.6}", (a + b + c) / 3.);
    println!("{:.6}", (a + b + d) / 3.);
    println!("{:.6}", (a + c + d) / 3.);
    println!("{:.6}", (b + c + d) / 3.);
    println!("{:.6}", (a + b + c + d) / 4.);
}
