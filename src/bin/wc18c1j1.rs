use std::io::BufRead;

/**
 * WC '18 Contest 1 J1 - Homework
 * Woburn Challenge 2018-19 Round 1 - Junior Division
 *
 * Alice is a student at H.S. High School. Right now, she's not the happiest student in the world, as she has a whole bunch
 * of math homework due tomorrow!
 *
 * Her homework consists of A ( 1 ≤ A ≤ 100 ) math questions, which Alice is supposed to complete one after another. Each
 * question takes M ( 1 ≤ M ≤ 100 ) minutes to complete.
 *
 * As important as Alice's homework is, she's had some more important shows to watch first, leaving her with only
 * T ( 1 ≤ T ≤ 100 ) minutes now remaining before her strict bedtime! She'd like to figure out whether she can complete all
 * A homework questions within at most T minutes, or if they would require a combined total of strictly more than T minutes
 * (in which case she'll use that time to come up with an excuse instead). Output Y if she still has time to finish her
 * homework today, or N if she doesn't.
 *
 * Input Specification
 * The first line of input consists of a single integer, A .
 * The next line consists of a single integer, M .
 * The next line consists of a single integer, T .
 *
 * Output Specification
 * Output a single character, either Y if Alice can complete all A assignments within at most T minutes, or N otherwise.
 *
 * Sample Input 1
 * 2
 * 3
 * 9
 *
 * Sample Output 1
 * Y
 *
 * Sample Input 2
 * 4
 * 3
 * 11
 *
 * Sample Output 2
 * N
 *
 * Sample Explanation
 * In the first case, the 2 assignments would require a total of 6 minutes, which is less than or equal to 9, meaning that
 * Alice has time to complete them.
 * In the second case, the 4 assignments would require a total of 12 minutes, which is greater than 11, meaning that Alice
 * won't have time to complete them.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let a: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    if a * m <= t {
        println!("Y");
    } else {
        println!("N");
    }
}
