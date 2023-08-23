use std::io::BufRead;

/**
 * DMOPC '14 Contest 4 P1 - New Key
 *
 * PRIORITY ONE TRANSMISSION
 *   Union of Ecraxus
 *   From: Agent Lore, Ecraxian Naval Intelligence, Section 2
 *   Subject: New Key
 *
 * Rumours are coming in from far sections of space that state the Hoyd aliens have deciphered our current system of coding
 * for secret messages. Though this has not yet been confirmed, we have decided to play it safe and implement a new system.
 *
 * In this new system, a letter is represented by something other than itself. Numbers 0–9 represent letters A–J (in order),
 * letters A–J represent the letters from K–T, while the letters K–P represent the letters U–Z.
 *
 * To make it more difficult to decipher, in the decryption procedure, each character must then be shifted one to the left
 * for the message to make sense. To decrypt 4 to D, 4 is changed to E, and then E is shifted to D. Important: when
 * decrypting, the input 0 (encrypted) becomes 9 (decrypted). It was shifted one to the left from A.
 *
 * We need you to make a key for this new code so that our agents will be able to communicate with one another.
 *
 * Input Specification
 * The only line of input will contain the encrypted message. It will have a length of up to 50 characters and will contain
 * only digits 0–9 and letters A–P, uppercase only.
 *
 * Output Specification
 * The only line of input should contain the decrypted message.
 *
 * Sample Input
 * 4DFG3
 *
 * Sample Output
 * DMOPC
 *
 * Explanation for Sample Output
 * The first character, 4 (0,1,2,3,4,5,6,7,8,9) becomes E (A,B,C,D,E,F,G,H,I,J). It is then shifted to the left by one to
 * obtain D, the decrypted character.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let result: String = line.chars().map(change).map(shift).collect();
    println!("{result}");
}

fn change(ch: char) -> char {
    let ch = ch as u8;

    let new_ch = match ch {
        b'0'..=b'9' => b'A' + (ch - b'0'),
        b'A'..=b'J' => b'K' + (ch - b'A'),
        b'K'..=b'P' => b'U' + (ch - b'K'),
        ch => ch,
    };

    new_ch as char
}

fn shift(ch: char) -> char {
    let ch = ch as u8;

    let new_ch = match ch {
        b'A' => b'9',
        b'B'..=b'Z' => ch - 1,
        ch => ch,
    };

    new_ch as char
}
