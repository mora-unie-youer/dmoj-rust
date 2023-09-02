use std::io::BufRead;

/**
 * DMOPC '16 Contest 2 P1 - Seed Strategy
 *
 * Kira is fighting the ZAFT forces in space, stopping chairman Durandal's plans to rule the Earth. Unfortunately, his mech,
 * the Strike Freedom has sustained major damages throughout the battle. Being the genius that he is, he calculates that he
 * has N minutes left before the Strike Freedom shuts down. Even worse, there are still K enemies nearby, with power levels
 * in the range [ 1 , 3 ] .
 *   Power level 1 grunts will take 30 seconds to defeat
 *   Power level 2 elites will take 1 minute to defeat
 *   Power level 3 aces will take 5 minutes to defeat
 *
 * Because he spent all his time calculating how much time he has left, he needs you to figure out if he has time to defeat
 * all the enemies or to make a tactical retreat.
 *
 * Input Specification
 * The first line of the input will contain an integer N ( 1 ≤ N ≤ 10 000 ) , representing the amount of time before the
 * Strike Freedom shuts down.
 *
 * The second line will contain an integer K , representing the number of enemies left.
 *
 * The next K ( 1 ≤ K ≤ 10 000 ) lines will each contain an element i ( 1 ≤ i ≤ 3 ) , representing the power level of that
 * enemy.
 *
 * Output Specification
 * If Kira can defeat all the enemies before his mech shuts down, output Continue. Otherwise, output Return.
 * Make sure your output matches this exactly, including capitalization.
 *
 * Sample Input
 * 10
 * 6
 * 1
 * 1
 * 2
 * 3
 * 1
 * 2
 *
 * Sample Output
 * Continue
 *
 * Explanation
 * We know that Kira has 10 minutes to defeat all 6 enemies. There are 3 Power level 1 enemies which is 3 × 0.5 = 1.5
 * minutes, 2 Power level 2 enemies which is 2 × 1 = 2 minutes and 1 Power level 3 enemy which is 1 × 5 = 5 minutes. In
 * total, it will take 1.5 + 2 + 5 = 8.5 minutes which is less than the 10 minutes given, therefore, Kira can continue to
 * fight.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    // Time in minutes
    let time: usize = lines.next().unwrap().unwrap().parse().unwrap();
    // Time in halfs of minute
    let time = time * 2;

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let required_time: usize = lines
        .take(n)
        .map(Result::unwrap)
        .filter_map(|line| line.chars().next())
        .map(|ch| match ch {
            '1' => 1,
            '2' => 2,
            '3' => 10,
            _ => unreachable!(),
        })
        .sum();

    if required_time < time {
        println!("Continue");
    } else {
        println!("Return");
    }
}
