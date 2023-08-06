use std::io::BufRead;

/**
 * DWITE '08 R1 #2 - Simple Checksum
 * DWITE Online Computer Programming Contest, October 2008, Problem 2
 *
 * A checksum is a type of simple error detection scheme, meant to catch incorrectly entered data, such as typos. Credit
 * cards, for example, use the Luhn algorithm to generate account numbers. Alternatively, a checksum number could be a digit
 * appended to the end of data that is being validated.
 *
 * A super-simple scheme used to validate 6 digit student numbers is as follows:
 *   Break the number up into 6 digits
 *   Add up all the digits together to get a new number
 *   Repeat the process until the result is only a single digit
 *   Match the resulting digit to the capital letter of the alphabet, in that position
 *
 * Example
 * 123456
 * 1+2+3+4+5+6 = 21
 * 2+1 = 3
 * 3 = C
 *
 * The input will contain 5 lines, 6-digit positive integers, followed by a space and a capital letter. Numbers will not
 * have leading zeros, and thus digits will never add up to 0 .
 * The output will contain 5 lines, stating match or error, depending if the number generates the same checksum letter as
 * supplied, or not.
 *
 * Sample Input
 * 123456 C
 * 123456 A
 * 100000 A
 * 111111 F
 * 111114 I
 *
 * Sample Output
 * match
 * error
 * match
 * match
 * match
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        let line = line.unwrap();
        let (input, checksum) = line.split_once(' ').unwrap();

        let checksum = checksum.chars().next().unwrap() as u8;

        let mut input_checksum: usize = input.bytes().map(|ch| (ch - b'0') as usize).sum();
        while input_checksum >= 10 {
            let mut new_checksum = 0;
            while input_checksum > 0 {
                new_checksum += input_checksum % 10;
                input_checksum /= 10;
            }

            input_checksum = new_checksum;
        }

        let input_checksum = b'A' + input_checksum as u8 - 1;
        if input_checksum == checksum {
            println!("match");
        } else {
            println!("error");
        }
    }
}
