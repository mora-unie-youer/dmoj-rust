use std::io::BufRead;

/**
Mock CCC '18 Contest 2 J1 - An Arithmetic Problem

You are given an arithmetic expression a + b = c . Determine if it is true or false.

Constraints
1 ≤ a , b , c ≤ 9
In a batch of tests worth 1 mark, the answer is always True.
In a potentially disjoint batch of tests worth 1 mark, the answer is always False.

Input Specification
The first line contains a single line of the form a + b = c, where each of a, b, and c are positive digits.

Output Specification
If the statement is true, print True on a single line.
Otherwise, print False on a single line.

Sample Input 1
1 + 1 = 2

Sample Output 1
True

Sample Input 2
1 + 1 = 3

Sample Output 2
False
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let line = line.as_bytes();

    let (a, b, c) = (line[0], line[4], line[8]);
    if a + b - b'0' == c {
        println!("True");
    } else {
        println!("False");
    }
}
