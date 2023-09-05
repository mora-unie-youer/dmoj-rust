use std::io::BufRead;

/**
 * CCC '06 J2 - Roll the Dice
 * Canadian Computing Competition: 2006 Stage 1, Junior #2
 *
 * Diana is playing a game with two dice. One die has m sides labelled 1 , 2 , 3 , … , m .
 *
 * The other die has n sides labelled 1 , 2 , 3 , … , n .
 *
 * Write a program to determine how many ways can she roll the dice to get the sum 10 .
 *
 * For example, when the first die has 6 sides and the second die has 8 sides, there are 5 ways to get the sum 10:
 *   2 + 8 = 10
 *   3 + 7 = 10
 *   4 + 6 = 10
 *   5 + 5 = 10
 *   6 + 4 = 10
 *
 * Input
 * The input is given as two integers. First, the user will enter in the number m ( 1 ≤ m ≤ 1000 ) .
 *
 * Second, the user will enter the number n ( 1 ≤ n ≤ 1000 ) .
 *
 * Output
 * The program prints out the number of ways 10 may be rolled on these two dice. Note that in the output, the word way
 * should be used if there is only one way to achieve the sum of 10 ; otherwise, the word ways should be used in the output.
 * That is, if there is only one way to get the sum 10 , the output should be:
 *   There is 1 way to get the sum 10.
 *
 * Sample Input 1
 * 6
 * 8
 *
 * Sample Output 1
 * There are 5 ways to get the sum 10.
 *
 * Sample Input 2
 * 12
 * 4
 *
 * Sample Output 2
 * There are 4 ways to get the sum 10.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut count = 0;
    for i in 1..=n {
        for j in 1..=m {
            count += (i + j == 10) as usize;
        }
    }

    if count == 1 {
        println!("There is 1 way to get the sum 10.");
    } else {
        println!("There are {count} ways to get the sum 10.");
    }
}
