use std::io::BufRead;

/**
 * BlueBook - Max
 *
 * Given T real numbers, find and output the largest one to four decimal places.
 *
 *
 * Input Specification
 * The first line contains a single integer T ( 1 ≤ T ≤ 1 000 000 ) . Each of the following T lines contains one real number
 * x_i ( −10^6 ≤ x_i ≤ 10^6 ).
 *
 * Output Specification
 * A single line: the largest of the T real numbers.
 *
 *
 * Sample Input:
 *   3
 *   1.5
 *   3
 *   2.2
 *
 * Sample Output:
 *   3.0000
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let max: f64 = lines
        .take(n)
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    println!("{:.4}", max);
}
