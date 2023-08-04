use std::{collections::VecDeque, io::BufRead};

/**
 * Mock CCC '20 Contest 2 S2 - 4D BBST on a DP
 *
 * You happened to find a queue of N lowercase latin characters in the mailbox. You decide you want to do something
 * productive with these characters. As such, you will try to build a string. For each character in the queue, in order, you
 * will either prepend or append the character to a new string, and then remove the character from the queue. Recall that a
 * queue is First-In First-Out (FIFO), meaning that the character at the front of the queue (the first character) is removed
 * first, second character second, etc.
 *
 * Given the queue of characters, what is the lexicographically smallest string that can be built?
 *
 * Input Specification
 * The first line will contain the integer N ( 1 ≤ N ≤ 10 5 ) , the number of characters in the queue.
 *
 * The second line will contain N lowercase latin characters concatenated together. The first character will be the first
 * character in the queue, second character the second character in the queue, etc.
 *
 * Output Specification
 * Output the lexicographically smallest string that can be built. This string should consist of exactly N characters.
 *
 * Subtasks
 * For 6/15 of the points, N ≤ 10
 *
 * Sample Input
 * 5
 * bzbsa
 *
 * Sample Output
 * abbzs
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut queue: VecDeque<u8> = lines.next().unwrap().unwrap().bytes().collect();

    let mut result = VecDeque::with_capacity(n);
    while let Some(ch) = queue.pop_front() {
        if result.front().map(|&first| ch <= first).unwrap_or(false) {
            result.push_front(ch);
        } else {
            result.push_back(ch);
        }
    }

    let result: String = result.into_iter().map(|ch| ch as char).collect();
    println!("{}", result);
}
