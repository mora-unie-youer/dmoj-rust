use std::io::BufRead;

/**
 * Appleby Contest '19 P2 - The Love Letter
 *
 * Plasmatic is sending a super special love letter, and because it is so special he doesn't want anyone (except his crush
 * obviously) to see it.
 *
 * To stop it from getting into the wrong hands, he has decided to encrypt the letter with a Caeser Cipher (don't worry his
 * crush will get it).
 *
 * Unfortunately, he isn't very skilled at programming and finds it difficult to apply the cipher to his letter. Can you
 * help him do it?
 *
 * Constraints
 * 1 ≤ N ≤ 10^4
 * 0 ≤ L ≤ 10^6
 *
 * Input Specification
 * The first line will contain the integer N , the length of the love letter.
 *
 * The second line will contain the integer L , the shift he wants to apply.
 *
 * The third line will contain the string S , the love letter itself. It will consist only of lowercase letters and spaces.
 *
 * Output Specification
 * Output the love letter with the cipher applied.
 *
 * Sample Input
 * 56
 * 2
 * roses are red violets are blue sugar is sweet so are you
 *
 * Sample Output
 * tqugu ctg tgf xkqngvu ctg dnwg uwict ku uyggv uq ctg aqw
 *
 * Explanation
 * Every letter in the input has been shifted two places forward as specified in the input. Note that the letter y was
 * looped back to a.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let _n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let l: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let s = lines.next().unwrap().unwrap();
    let encrypted: String = s.chars().map(|ch| rot(ch, l)).collect();
    println!("{encrypted}");
}

fn rot(ch: char, l: usize) -> char {
    if ch == ' ' {
        return ' ';
    }

    let i = (ch as u8 - b'a') as usize;
    let rotated = (i + l) % 26;
    (rotated as u8 + b'a') as char
}
