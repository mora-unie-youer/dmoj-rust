use std::io::BufRead;

/**
 * DMPG '16 B1 - BFS
 *
 * Bob is your ordinary university student who waits tables at the local restaurant. However, Bob (as with all aspiring
 * adolescents) is burdened with his crushing student debt and due to the stress, he cannot exhibit the maximum potential of
 * his academic performance. To relieve himself of this stress, Bob engages in various recreational activities. One such
 * activity is counting money.
 *
 * Bob's First Salary is a very important milestone in his life. Luckily, Bob received his tips and wages in Canadian
 * Currency meaning he can run home and count his money over and over again!
 *
 * Input Specification
 * Five space-separated integers: N , D , Q , L , T indicating the quantity of each currency Bob received from work
 * ( 0 ≤ N , D , Q , L , T ≤ 10 ) .
 *
 * These variables represent the collective amount of money Bob received in nickels, dimes, quarters, loonies, and toonies
 * respectively.
 *   Coins       N  D   Q   L    T
 *   Value (¢)   5  10  25  100  200
 *
 * Output Specification
 * Your program should output the value of Bob's First Salary in cents.
 *
 * Sample Input
 * 0 0 0 1 0
 *
 * Sample Output
 * 100
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    const VALUES: [usize; 5] = [5, 10, 25, 100, 200];
    let input = lines.next().unwrap().unwrap();

    let cents = input
        .split_ascii_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .zip(VALUES)
        .fold(0, |acc, (amount, value)| acc + amount * value);
    println!("{cents}");
}
