use std::io::BufRead;

/**
 * BlueBook - Change
 *
 * Write a program that makes change for amounts less than one dollar with the least number of coins.
 *
 * Input Specification
 * Input will be a positive integer less than 100 , representing an amount of money in cents.
 *
 * Output Specification
 * Output should be the original amount of money together with a minimal set of coins (quarters, dimes, nickels, cents) that
 * could make up that amount.
 *
 * Sample Input
 * 58
 *
 * Sample Output
 * 58 cents requires 2 quarters, 1 nickel, 3 cents.
 *
 * Note:
 * "58 cents requires 2 quarters, 0 dimes, 1 nickels, 3 cents." will not be accepted.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    if n == 1 {
        println!("1 cent requires 1 cent.");
        return;
    }

    let quarters = n / 25;
    let dimes = (n - quarters * 25) / 10;
    let nickels = (n - (quarters * 25 + dimes * 10)) / 5;
    let cents = n - (quarters * 25 + dimes * 10 + nickels * 5);

    let mut parts = Vec::new();

    if quarters > 0 {
        let mut s = format!("{quarters} quarter");
        if quarters != 1 {
            s.push('s');
        }
        parts.push(s);
    }

    if dimes > 0 {
        let mut s = format!("{dimes} dime");
        if dimes != 1 {
            s.push('s');
        }
        parts.push(s);
    }

    if nickels > 0 {
        let mut s = format!("{nickels} nickel");
        if nickels != 1 {
            s.push('s');
        }
        parts.push(s);
    }

    if cents > 0 {
        let mut s = format!("{cents} cent");
        if cents != 1 {
            s.push('s');
        }
        parts.push(s);
    }

    let parts = parts.join(", ");
    println!("{n} cents requires {parts}.");
}
