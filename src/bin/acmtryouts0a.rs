use std::io::BufRead;

/**
 * ACM U of T Tryouts C0 A - Max Flow
 * University of Toronto ACM-ICPC Tryouts 2012
 *
 * Many computer scientists have nightmares about the daunting task of finding the max flow. Can you handle it?
 *
 * There are T ( 1 ≤ T ≤ 10 ) scenarios. In each scenario, there are N ( 1 ≤ N ≤ 10 ) flows, and the value of the i -th flow
 * is F i ( 1 ≤ F i ≤ 100 ) - your job is to find the largest of the flow values.
 *
 * Input Specification
 * Line 1: 1 integer, T
 * For each scenario:
 * Line 1: 1 integer, N
 * Next N lines: 1 integer, F i , for i = 1 … N
 *
 * Output Specification
 * For each scenario:
 * Line 1: The largest flow value.
 *
 * Sample Input
 * 2
 * 4
 * 2
 * 5
 * 3
 * 5
 * 1
 * 1
 *
 * Sample Output
 * 5
 * 1
 *
 * Explanation of Sample
 * In the first scenario, the 4 flows have values of 2, 5, 3, and 5, respectively. The largest of these values is 5.
 * In the second scenario, the only flow has a value of 1, so the max flow is 1.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let max: usize = lines
            .by_ref()
            .take(n)
            .map(Result::unwrap)
            .map(|v| v.parse().unwrap())
            .max()
            .unwrap();
        println!("{max}");
    }
}
