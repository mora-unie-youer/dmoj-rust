use std::io::BufRead;

/**
 * DWITE '11 R5 #1 - Age Gate
 * DWITE Online Computer Programming Contest, October 2010, Problem 1
 *
 * DWITE has traditionally been a high school level contest, so to ensure that no middle schoolers try to sneak in and make
 * older students feel uncomfortable, an age gate is being considered.
 *
 * The input will contain 5 lines, each having 3 integers representing a person's birth date, in the form of DD MM YYYY
 * (separated by spaces).
 *
 * The output will contain 5 lines of output, decisions if the participant is at least 13 years old as of 27 / 10 / 2010 or
 * not. Print old enough or too young depending on the age.
 *
 * Sample Input
 * 28 10 1997
 * 27 10 1997
 * 26 11 1997
 *
 * Sample Output
 * too young
 * old enough
 * too young
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    for line in lines.take(5) {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace().map(|v| v.parse().unwrap());
        let day: usize = line.next().unwrap();
        let month = line.next().unwrap();
        let year = line.next().unwrap();

        let young = year
            .cmp(&1997)
            .then(month.cmp(&10))
            .then(day.cmp(&27))
            .is_gt();

        if young {
            println!("too young");
        } else {
            println!("old enough");
        }
    }
}
