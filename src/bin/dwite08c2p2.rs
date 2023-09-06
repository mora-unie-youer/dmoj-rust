use std::io::BufRead;

/**
 * DWITE '08 R2 #2 - My favourite digit
 * DWITE Online Computer Programming Contest, November 2008, Problem 2
 *
 * Simply speaking, you need to know how to handle your numbers well. Given an integer, what is the largest digit present in that number?
 *
 * The input will contain 5 lines, integers 0 ≤ N ≤ 1 000 000 .
 * The output will contain 5 lines, each a single integer 0 ≤ n ≤ 9 – the largest digit present in the corresponding input number.
 *
 * Sample Input
 * 1
 * 10
 * 102
 * 1025
 * 9999
 *
 * Sample Output
 * 1
 * 1
 * 2
 * 5
 * 9
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    for line in lines.take(5) {
        let line = line.unwrap();
        let max_digit = line.bytes().max().unwrap();
        let max_digit = max_digit as char;
        println!("{}", max_digit);
    }
}
