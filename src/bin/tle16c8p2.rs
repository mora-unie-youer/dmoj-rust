use std::io::BufRead;

/**
 * TLE '16 Contest 8 P2 - Dank Meme
 *
 * Fax McClad, Croneria's smartest bounty hunter, is playing a game of Dank Meme with his wingmate Flaco so that he can keep
 * his thinking skills sharp.
 *
 * In Dank Meme, players take turns reciting integers in their binary representations without leading zeros, but they
 * replace every instance of 1 with dank and every instance of 0 with meme.
 *
 * It's Fax's turn to recite a number, and it happens to be N . Fax can't seem to figure out how to recite this number. Can
 * you tell him?
 *
 * Input Specification
 * The first line of input will contain T ( 1 ≤ T ≤ 1 000 ) , the number of times Fax will recite a number.
 *
 * T lines of input will follow. Each line will contain N ( 0 ≤ N ≤ 10 9 ) .
 *
 * Output Specification
 * For each of the T inputs, output a single line of space-separated strings specifying what Fax should recite.
 *
 * Sample Input
 * 2
 * 3
 * 5
 *
 * Sample Output
 * dank dank
 * dank meme dank
 *
 * Explanation for Sample Output
 * The binary representation of 3 is 11 .
 * The binary representation of 5 is 101 .
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let number: usize = line.unwrap().parse().unwrap();
        let number = format!("{number:b}");

        let words: Vec<&str> = number
            .chars()
            .map(|ch| ch == '1')
            .map(|dank| if dank { "dank" } else { "meme" })
            .collect();
        println!("{}", words.join(" "));
    }
}
