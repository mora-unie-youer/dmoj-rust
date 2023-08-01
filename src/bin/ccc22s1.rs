use std::io::BufRead;

/**
 * CCC '22 S1 - Good Fours and Good Fives
 * Canadian Computing Competition: 2022 Stage 1, Senior #1
 *
 * Finn loves Fours and Fives. In fact, he loves them so much that he wants to know the number of ways a number can be
 * formed by using a sum of fours and fives, where the order of the fours and fives does not matter. If Finn wants to form
 * the number 14 , there is one way to do this which is 14 = 4 + 5 + 5 . As another example, if Finn wants to form the
 * number 20 , this can be done two ways, which are 20 = 4 + 4 + 4 + 4 + 4 and 20 = 5 + 5 + 5 + 5 . As a final example,
 * Finn can form the number 40 in three ways: 40 = 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 ,
 * 40 = 4 + 4 + 4 + 4 + 4 + 5 + 5 + 5 + 5 , and 40 = 5 + 5 + 5 + 5 + 5 + 5 + 5 + 5 .
 *
 * Your task is to help Finn determine the number of ways that a number can be written as a sum of fours and fives.
 *
 * Input Specification
 * The input consists of one line containing a number N .
 *
 * The following table shows how the available 15 marks are distributed.
 * Marks Awarded  Bounds on N        Additional Constraints
 * 3 marks        1 ≤ N ≤ 10         None
 * 2 marks        1 ≤ N ≤ 100 000    N is a multiple of 4
 * 2 marks        1 ≤ N ≤ 100 000    N is a multiple of 5
 * 8 marks        1 ≤ N ≤ 1 000 000  None
 *
 * Output Specification
 * Output the number of unordered sums of fours and fives which form the number N . Output 0 if there are no such sums of
 * fours and fives.
 *
 * Sample Input 1
 * 14
 *
 * Output for Sample Input 1
 * 1
 *
 * Explanation of Output for Sample Input 1
 * This is one of the examples in the problem description.
 *
 * Sample Input 2
 * 40
 *
 * Output for Sample Input 2
 * 3
 *
 * Explanation of Output for Sample Input 2
 * This is one of the examples in the problem description.
 *
 * Sample Input 3
 * 6
 *
 * Output for Sample Input 3
 * 0
 *
 * Explanation of Output for Sample Input 3
 * There is no way to use a sum of fours and fives to get 6 .
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut number: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut sums = (number % 5 == 0) as usize;

    while let Some(n) = number.checked_sub(4) {
        sums += (n % 5 == 0) as usize;
        number = n;
    }

    println!("{sums}");
}
