use std::io::BufRead;

/**
 * Spring Coding Bowl '22 P1 - Student Numbers
 *
 * At your school, every student has a student number. A student number is a 6 digit long sequence that is unique to them,
 * which was typed on the following keyboard:
 * 1 2 3 4 5 6 7 8 9 0
 *
 * The first digit of a student number can start on any of the digits above. However, each successive digit must be EXACTLY
 * one key to the left or right of its previous digit on the keyboard. For example, the student number 123454 is valid
 * because each successive number is one to the left or right of its previous digit on the keyboard. However, the student
 * number 123453 is NOT valid. Namely, the final digit 3 is 2 keys away from the digit 5 on the keyboard despite being
 * directly after 5 in the student number.
 *
 * Your task is to build a program that determines if the given student number is valid or invalid.
 *
 * Input Specification
 * The input consists of one line containing a string of exactly 6 digits representing a student number.
 *
 * Output Specification
 * If the given student number meets the requirements, print VALID. Otherwise, print INVALID.
 *
 * Sample Input 1
 * 123454
 *
 * Sample Output 1
 * VALID
 *
 * Explanation for Sample 1
 * This is one of the examples in the problem description.
 *
 * Sample Input 2
 * 123453
 *
 * Sample Output 2
 * INVALID
 *
 * Explanation for Sample 2
 * This is one of the examples in the problem description.
 *
 * Sample Input 3
 * 890109
 *
 * Sample Output 3
 * INVALID
 *
 * Explanation for Sample 3
 * The digits 0 and 1 are on opposite sides of the keyboard, which does not count as being adjacent.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let number = lines.next().unwrap().unwrap();
    let number = number.as_bytes();

    let is_valid = number.windows(2).all(|chunk| {
        let (a, b) = (position(chunk[0]), position(chunk[1]));
        a.abs_diff(b) == 1
    });

    if is_valid {
        println!("VALID");
    } else {
        println!("INVALID");
    }
}

fn position(digit: u8) -> u8 {
    (10 + b'0' - digit) % 10
}
