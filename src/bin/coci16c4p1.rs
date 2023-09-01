use std::io::BufRead;

/**
 * COCI '16 Contest 4 #1 Bridž
 *
 * After a few months of playing on his new phone, Mirko has finally decided to find a new hobby. He discovered a card game
 * called bridge!
 *
 * It is known that bridge is played by four players using a 52 -card deck, 4 suits total (clubs, hearts, diamonds and
 * spades) and 13 values (A, K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3 and 2). At the beginning of the game, each player is dealt 13
 * cards.
 *
 * Before starting the game, each player counts their so-called honor points in the following way:
 *   each ace (A) is worth 4 points
 *   each king (K) is worth 3 points
 *   each queen (Q) is worth 2 points
 *   each jack (J) is worth 1 point
 *   the remaining cards (that will be denoted with X in this task) are worth 0 points.
 *
 * Given the fact that Mirko has started playing bridge only recently, he has decided to practice counting points. He dealt
 * himself cards N times and each time counted his honor points. In the end, he added them up.
 *
 * He wants to know if he's done a good job. Help Mirko and check!
 *
 * Input Specification
 * The first line of input contains the integer N ( 1 ≤ N ≤ 10 000 ) from the task.
 *
 * Each of the following N lines contains K i , a string consisting of characters A, K, Q, J, X, of length 13 , representing
 * the cards Mirko had in his hand after dealing them for the i th time.
 *
 * Output Specification
 * The first and only line of output must contain the required sum from the task.
 *
 * Sample Input 1
 * 1
 * AKXAKJXXXAXAQ
 *
 * Sample Output 1
 * 25
 *
 * Explanation for Sample Output 1
 * Mirko has a total of 4 aces, 2 kings, 1 queen and 1 jack in his hand. This totals to 4 × 4 + 2 × 3 + 1 × 2 + 1 × 1 = 25
 * honor points.
 *
 * Sample Input 2
 * 4
 * XXXAXXXXXXJXX
 * KXAXXXQJAXXXX
 * AQKQXXXKXXKQX
 * JXXXXXJXXXXXX
 *
 * Sample Output 2
 * 40
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let honor: usize = lines
        .take(n)
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'A' => 4,
                    'K' => 3,
                    'Q' => 2,
                    'J' => 1,
                    'X' => 0,
                    _ => unreachable!(),
                })
                .sum::<usize>()
        })
        .sum();

    println!("{honor}");
}
