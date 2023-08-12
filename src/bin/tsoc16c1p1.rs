use std::io::BufRead;

/**
 * TSOC '16 Contest 1 #1 - Vasile and Haircuts
 *
 * Vasile runs a barber shop. His customers tell him how long their hair is before he cuts it, and how much they want cut
 * off.
 *
 * Determine whether the length after the cut is inside the acceptable inclusive range.
 *
 * Input Specification
 * The first line will contain an integer N , the number of haircuts he gives.
 *
 * The following N lines contain 4 numbers. Each line represents a customer's order, where there will be four
 * space-separated integers in the format L A B F, the initial length of the customer's hair, the minimum they want taken
 * off, the maximum they want taken off, and the final length of their hair.
 *
 * Output Specification
 * N lines, the i th being Yes if the final length is within the acceptable range, or No if it isn't in the acceptable
 * range.
 *
 * Sample Input
 * 5
 * 20 5 10 12
 * 10 2 4 8
 * 15 3 5 10
 * 3 1 2 3
 * 20 2 3 1
 *
 * Sample Output
 * Yes
 * Yes
 * Yes
 * No
 * No
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        let params: Vec<usize> = line
            .split_ascii_whitespace()
            .take(4)
            .map(|v| v.parse().unwrap())
            .collect();

        if (params[0] - params[2]..=params[0] - params[1]).contains(&params[3]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
