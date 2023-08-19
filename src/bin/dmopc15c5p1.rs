use std::io::BufRead;

/**
 * DMOPC '15 Contest 5 P1 - Gel Bananas
 *
 * Okabe and Kurisu are experimenting on bananas. Okabe starts a new experiment every A days, and Kurisu starts a new
 * experiment every B days. Today, on day 1 , Okabe and Kurisu both start an experiment. How many times will Okabe and
 * Kurisu start their experiments together in X days?
 *
 * Input Specification
 * The first, second and third lines of input will contain A , B ( 1 ≤ A , B ≤ 10^9 ) , and X ( 1 ≤ X ≤ 10^18 ),
 * respectively.
 *
 * Output Specification
 * Output a single integer, the number of times that Okabe and Kurisu start their experiments together.
 *
 * Sample Input
 * 2
 * 3
 * 10
 *
 * Sample Output
 * 2
 *
 * Explanation
 * Okabe will start his experiments on days 1 , 3 , 5 , 7 , 9 and Kurisu will start her experiments on
 * days 1 , 4 , 7 , 10 . They start their experiments together on days 1 and 7 .
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let a: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let b: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let x: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let lcm = a * b / gcd(a, b);
    let common_days = (x - 1) / lcm + 1;

    println!("{common_days}");
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    if m < n {
        std::mem::swap(&mut m, &mut n);
    }

    while n != 0 {
        m %= n;
        std::mem::swap(&mut m, &mut n);
    }

    m
}
