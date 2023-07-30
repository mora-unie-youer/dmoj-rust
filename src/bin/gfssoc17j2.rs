use std::io::BufRead;

/**
 * GFSSOC '17 J2 - Admission Average
 *
 * Ace is applying to the University of Waterloo Software Engineering and he wants to know whether his top 6 average and
 * Admission Information Form. Given his top six integer grades, ( 1 ≤ grade ≤ 100 ) and an integer AIF score
 * ( 0 ≤ score ≤ 5 ) , determine if he can get in. The average is the average of the top 6 grades plus the AIF score.
 * Waterloo sets the minimum integer admission requirement ( 0 ≤ r e q u i r e m e n t ≤ 105 ) . Output yes if Ace can make
 * it. Otherwise, output no.
 *
 * Input Specification
 * The first six lines of input are his six integer marks.
 * The next line is the integer AIF score.
 * The final line is the integer admission requirement.
 *
 * Output Specification
 * yes if Ace can get into Waterloo Software Engineering, no if he cannot.
 *
 * Sample Input 1
 * 95
 * 96
 * 92
 * 88
 * 98
 * 94
 * 5
 * 96
 *
 * Sample Output 1
 * yes
 *
 * Sample Input 2
 * 75
 * 79
 * 69
 * 72
 * 88
 * 90
 * 3
 * 82
 *
 * Sample Output 2
 * no
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let input: Vec<usize> = lines
        .take(8)
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect();
    let (grades, aif, requirement) = (&input[0..6], input[6], input[7]);
    let average = grades.iter().sum::<usize>() / grades.len();

    if average + aif >= requirement {
        println!("yes");
    } else {
        println!("no");
    }
}
