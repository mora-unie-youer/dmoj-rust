use std::io::BufRead;

/**
 * BlueBook - Lower Case
 *
 * Given an input string, whose length is less than 256 , output the string with all of the upper case letters (A…Z)
 * converted to the corresponding lower case letter (a…z).
 *
 * Input Specification
 * Line 1 : one integer T ( 1 ≤ T ≤ 100 ) denoting the number of test cases.
 * Lines 2 … T + 1 : one string of length < 256 composed of lower case letters (a…z), upper case letters (A…Z), and random
 * characters (*, ;, etc).
 *
 * Sample Input
 * 2
 * abCDefg
 * WEEEIPWNNOOBS~~?~?~??@#!#$%?$^?$%*
 *
 * Sample Output
 * abcdefg
 * weeeipwnnoobs~~?~?~??@#!#$%?$^?$%*
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for line in lines.take(n) {
        let line = line.unwrap();
        println!("{}", line.to_ascii_lowercase());
    }
}
