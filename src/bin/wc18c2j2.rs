use std::io::BufRead;

/**
 * WC '18 Contest 2 J2 - This Message will Self-Destruct
 * Woburn Challenge 2018-19 Round 2 - Junior Division
 *
 * Ethan Hunt has received an audio message recorded by Alan Hunley, the secretary of the IMF, detailing a secret upcoming
 * mission to Newfoundland. Ethan is well aware of a security protocol dictating that the message must self-destruct shortly
 * after being heard, in order to prevent its information from falling into enemy hands. Normally, Alan would end the
 * message by mentioning how quickly it will self-destruct, but it appears that he forgot to do so this time!
 *
 * Fortunately, the message did come with a digital clock display instead, which is ticking downwards. Ethan assumes that
 * the message will self-destruct when the clock reaches 0 : 00 .
 *
 * The current clock display may be represented by a string S with exactly four characters, in the format m:ss. The first
 * character (m) is a digit 0…9, and indicates the number of minutes remaining. The second character is always :. The last
 * two characters (ss) form a number (00…59), indicating the number of seconds additionally remaining. Specifically, the
 * third character is a digit 0…5, while the fourth character is a digit 0…9. It's guaranteed that S is not already equal to
 * 0:00.
 *
 * Based on the clock display, help Ethan determine how many seconds he has left to get the message off his hands before it
 * self-destructs!
 *
 * Input Specification
 * The first and only line of input consists of a single string, S (having exactly 4 characters, in the format m:ss).
 *
 * Output Specification
 * Output a single integer, the number of seconds remaining until the message self-destructs.
 *
 * Sample Input
 * 2:07
 *
 * Sample Output
 * 127
 *
 * Sample Explanation
 * The message will self-destruct in 2 minutes and 7 seconds, which is equivalent to 127 seconds.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let (minutes, seconds) = line.split_once(':').unwrap();
    let (minutes, seconds): (usize, usize) = (minutes.parse().unwrap(), seconds.parse().unwrap());
    println!("{}", minutes * 60 + seconds);
}
