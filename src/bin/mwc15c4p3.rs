use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

/**
 * MWC '15 #4 P3: Salt
 *
 * After failing his accounting, physics and engineering tests all in one day, aurpine has decided to give you a problem!
 * The problem is as follows.
 *
 * There are N grains of salt, labelled 1 to N . The n th grain is located at the coordinate ( X n , Y n ) . Two grains
 * won't occupy the same coordinate (because that's crazy!).
 *
 * You are to answer Q queries. There are two types of queries.
 *   1 x y – if there is a piece of salt at ( x , y ) output salty, otherwise output bland.
 *   2 X x – output the number of pieces of salt with an x-coordinate of x .
 *   2 Y y – output the number of pieces of salt with a y-coordinate of y .
 *
 *
 * Input Specification
 * Input will initiate with two space separated integers N and Q on a single line.
 *
 * N lines follow with two space separated integers, X n and Y n , the coordinate of the n th grain of salt.
 *
 * Q lines follow, in the queries form explained above.
 *
 * Note: fast input may be required.
 *
 * Constraints
 * Subtask 1 [10%]
 * 1 ≤ N , Q ≤ 100
 * 1 ≤ X n , Y n ≤ 10^3
 *
 * Subtask 2 [30%]
 * 1 ≤ N , Q ≤ 10 000
 * 1 ≤ X n , Y n ≤ 10^3
 *
 * Subtask 3 [60%]
 * 1 ≤ N , Q ≤ 10 000
 * 1 ≤ X n , Y n ≤ 10^9
 *
 * Output Specification
 * Q lines, one for each query.
 *
 * Sample Input
 * 5 4
 * 1 2
 * 3 5
 * 4 3
 * 4 5
 * 4 7
 * 1 2 1
 * 1 1 2
 * 2 X 4
 * 2 Y 5
 *
 * Sample Output
 * bland
 * salty
 * 3
 * 2
 *
 * Explanation for Sample Output
 * There is no grain of salt at ( 2 , 1 ) . There is a grain of salt at ( 1 , 2 ) . There are 3 grains with an x-coordinate
 * of 4. There are 2 grains with a y-coordinate of 5.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, q) = first_line.split_once(' ').unwrap();
    let (n, q): (usize, usize) = (n.parse().unwrap(), q.parse().unwrap());

    let mut grains = HashSet::new();
    let mut xs = HashMap::new();
    let mut ys = HashMap::new();

    for line in lines.by_ref().take(n) {
        let line = line.unwrap();
        let (x, y) = line.split_once(' ').unwrap();
        let grain @ (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());

        grains.insert(grain);
        *xs.entry(x).or_insert(0usize) += 1;
        *ys.entry(y).or_insert(0usize) += 1;
    }

    for line in lines.take(q) {
        let line = line.unwrap();
        let mut parts = line.split_ascii_whitespace();

        let ty = parts.next().unwrap();
        if ty == "1" {
            let mut coords = parts.map(|v| v.parse().unwrap());
            let grain = (coords.next().unwrap(), coords.next().unwrap());

            if grains.contains(&grain) {
                println!("salty");
            } else {
                println!("bland");
            }
        } else {
            let coord_ty = parts.next().unwrap();
            let coord = parts.next().unwrap().parse().unwrap();

            if coord_ty == "X" {
                println!("{}", xs.get(&coord).unwrap_or(&0));
            } else {
                println!("{}", ys.get(&coord).unwrap_or(&0));
            }
        }
    }
}
