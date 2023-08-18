use std::io::BufRead;

/**
 * DWITE '07 R5 #1 - I heart you
 * DWITE Online Computer Programming Contest, February 2008, Problem 1
 *
 * Just in time for Valentine's Day 2008, we require an ASCII heart generator. Based on Tony's horrible "artwork" below:
 *  ~.~
 * `   `
 *  \./
 *
 * The input will contain 5 lines, integers 0 ≤ N ≤ 5 .
 * The output will contain 5 sets of 3 lines, 15 in total. Each set will have a specified number of hearts generated. Refer
 * to the sample output for examples.
 * Note: each heart is made up of 3 lines, 5 characters each. Don't forget to have all the spaces at the end.
 *
 * Sample Input
 * 1
 * 0
 * 3
 *
 * Sample Output
 *  ~.~
 * `   `
 *  \./
 *
 *
 *
 *  ~.~  ~.~  ~.~
 * `   ``   ``   `
 *  \./  \./  \./
 */
fn main() {
    const PATTERN: [&str; 3] = [" ~.~ ", "`   `", r" \./ "];

    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    for line in lines.take(5) {
        let line = line.unwrap();
        let n: usize = line.parse().unwrap();

        for pattern in PATTERN {
            let mut line = String::new();
            for _ in 0..n {
                line.push_str(pattern);
            }

            println!("{line}");
        }
    }
}
