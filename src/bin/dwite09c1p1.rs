use std::io::BufRead;

/**
 * DWITE '09 R1 #1 - iProfits
 * DWITE Online Computer Programming Contest, October 2009, Problem 1
 *
 * Having decided to capitalize on your awesome programming skills, you've set out to create and sell a mobile application
 * at $ 0.99 per copy. Since the application is hosted and distributed through a managed platform, the store gets to keep
 * 30 % from each sale.
 *
 * Given that you have an idea of how much profit you want to make off your hard work, at least how many 1 000 s of copies
 * must be sold? (That is, the answer is rounded to the next 1 000 ).
 *
 * The input will contain 5 lines, integers 0 ≤ N ≤ 1 000 000 , the minimum profit you want to keep.
 *
 * The output will contain 5 lines, integer value of the number of copies needed to be sold, rounded to the next 1 000.
 *
 * For example: if you want to make $ 1 000 , then ( 1 444  copies × $ 0.99 ) − 30 % = $ 1 000.692 . ( 1 443 copies will
 * earn below $ 1 000 ). 1 444 rounded to the next 1 000 is 2 000 ; thus the answer is 2 000 .
 *
 * Sample Input
 * 0
 * 1
 * 693
 * 694
 * 250000
 *
 * Sample Output
 * 0
 * 1000
 * 1000
 * 2000
 * 361000
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    for line in lines.take(5) {
        let line = line.unwrap();

        let profit: f64 = line.parse().unwrap();
        let mut copies = (profit / (0.99 * 0.7)) as usize;
        // If it's not enough, adding one copy
        copies += (copies as f64 * 0.99 * 0.7 < profit) as usize;

        // Rounding up
        let copies = (copies + 999) / 1000 * 1000;
        println!("{copies}");
    }
}
