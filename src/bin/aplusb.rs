use std::io::BufRead;

/**
 * A Plus B
 *
 * Tudor is sitting in math class, on his laptop. Clearly, he is not paying attention in this situation. However, he gets
 * called on by his math teacher to do some problems. Since his math teacher did not expect much from Tudor, he only
 * needs to do some simple addition problems. However, simple for you and I may not be simple for Tudor, so please help him!
 *
 * Input Specification
 * The first line will contain an integer N ( 1 ≤ N ≤ 100 000 ) , the number of addition problems Tudor needs to do.
 * The next N lines will each contain two space-separated integers whose absolute value is less than 1 000 000 000, the two
 * integers Tudor needs to add.
 *
 * Output Specification
 * Output N lines of one integer each, the solutions to the addition problems in order.
 *
 * Sample Input:
 *   2
 *   1 1
 *   -1 0
 *
 * Sample Output:
 *   2
 *   -1
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let (a, b) = line.split_once(' ').unwrap();
        let (a, b): (isize, isize) = (a.parse().unwrap(), b.parse().unwrap());
        println!("{}", a + b);
    }
}
