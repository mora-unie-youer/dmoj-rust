use std::io::BufRead;

/**
 * Google Code Jam '15 Qualification Round Problem A - Standing Ovation
 *
 * It's opening night at the opera, and your friend is the prima donna (the lead female singer). You will not be in the
 * audience, but you want to make sure she receives a standing ovation -- with every audience member standing up and
 * clapping their hands for her.
 *
 * Initially, the entire audience is seated. Everyone in the audience has a shyness level. An audience member with shyness
 * level S i will wait until at least S i other audience members have already stood up to clap, and if so, she will
 * immediately stand up and clap. If S i = 0 , then the audience member will always stand up and clap immediately,
 * regardless of what anyone else does. For example, an audience member with S i = 2 will be seated at the beginning, but
 * will stand up to clap later after she sees at least two other people standing and clapping.
 *
 * You know the shyness level of everyone in the audience, and you are prepared to invite additional friends of the prima
 * donna to be in the audience to ensure that everyone in the crowd stands up and claps in the end. Each of these friends
 * may have any shyness value that you wish, not necessarily the same. What is the minimum number of friends that you need
 * to invite to guarantee a standing ovation?
 *
 * Input Specification
 * The first line of the input gives the number of test cases, T . T test cases follow. Each consists of one line with Smax,
 * the maximum shyness level of the shyest person in the audience, followed by a string of S max + 1 single digits. The k th
 * digit of this string (counting starting from 0) represents how many people in the audience have shyness level k . For
 * example, the string "409" would mean that there were four audience members with S i = 0 and nine audience members with
 * S i = 2 (and none with S i = 1 or any other value). Note that there will initially always be between 0 and 9 people with
 * each shyness level.
 *
 * The string will never end in a 0. Note that this implies that there will always be at least one person in the audience.
 *
 * Output Specification
 * For each test case, output one line containing Case #x: y, where x is the test case number (starting from 1) and y is the
 * minimum number of friends you must invite.
 *
 * Limits
 *   Memory limit: 1 GB.
 *   1 ≤ T ≤ 100 .
 *   
 *   Small dataset
 *   Time limit: 30 seconds.
 *   0 ≤ S max ≤ 6 .
 *   
 *   Large dataset
 *   Time limit: 60 seconds.
 *   0 ≤ S max ≤ 1 000 .
 *
 * Sample Input
 *   4
 *   4 11111
 *   1 09
 *   5 110011
 *   0 1
 *
 * Sample Output
 *   Case #1: 0
 *   Case #2: 1
 *   Case #3: 2
 *   Case #4: 0
 *
 * In Case #1, the audience will eventually produce a standing ovation on its own, without you needing to add anyone --
 * first the audience member with S i = 0 will stand up, then the audience member with S i = 1 will stand up, etc.
 *
 * In Case #2, a friend with S i = 0 must be invited, but that is enough to get the entire audience to stand up.
 *
 * In Case #3, one optimal solution is to add two audience members with S i = 2 .
 *
 * In Case #4, there is only one audience member and he will stand up immediately. No friends need to be invited.
 *
 * Note
 * This problem has different time limits for different batches. If you exceed the Time Limit for any batch, the judge will
 * incorrectly display >60.000s regardless of the actual time taken. Refer to the Limits section for batch-specific time
 * limits.
 *
 * This problem originally had a much higher time limit. However, as reference solutions were much faster, the Time Limit
 * was been reduced accordingly.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for (i, line) in lines.take(t).enumerate() {
        let line = line.unwrap();
        let (_smax, people) = line.split_once(' ').unwrap();

        let people = people.as_bytes();

        let mut friends = 0;
        let mut stand = (people[0] - b'0') as usize;
        for (i, shy_people) in people.iter().enumerate().skip(1) {
            if i > stand {
                friends += i - stand;
                stand += i - stand;
            }

            stand += (shy_people - b'0') as usize;
        }

        println!("Case #{}: {}", i + 1, friends);
    }
}
