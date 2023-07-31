use std::io::BufRead;

/**
 * Cheerio Contest 3 P2 - Double-O-Seven
 *
 * You are playing a game of "Double-O-Seven" with your friend. In the game, both players start with an ammo count and a
 * score of 0 . The game is played in N rounds. In a round, both players simultaneously choose one of three actions:
 *   R Reload: The player increases their ammo count by 1 .
 *   F Fire:
 *   If the player's ammo count is 0 , firing does nothing.
 *   If the player's ammo count is greater than 0 , then:
 *     Decrease the player's ammo count by 1 .
 *     Decrease the opponent's score by 1 .
 *     Increase the player's score by 1 .
 *   B Block: Restrict both players' score from increasing or decreasing this round.
 *
 * Your friend is so confident that he could beat you, that he will tell you all his actions beforehand. What is the
 * maximum score you could get, knowing your opponent's actions?
 *
 * Constraints
 * Points Awarded      N         Additional Constraints
 * 2 points        1 ≤ N ≤ 10^3  The opponent never fires
 * 3 points        1 ≤ N ≤ 10^3  None
 * 10 points       1 ≤ N ≤ 10^6  None
 *
 * Input Specification
 * The first line contains a single integer N .
 * The second line contains a string of length N consisting of the characters RFB. The i th character in the string
 * indicates the action your friend will make on the i th round of the game.
 *
 * Output Specification
 * Output the maximum possible score you can obtain if you played optimally.
 *
 * Sample Input 1
 * 3
 * RFF
 *
 * Sample Output 1
 * 1
 *
 * Explanation for Sample Output 1
 * A possible sequence of actions that obtains this score is RBF.
 * On the first round, both players reload and have an ammo count of 1 .
 * On the second round, your friend fires and decreases their ammo count to 0 . Since you blocked, both players still have a
 * score of 0 .
 * On the third round, your friend does nothing as they fired with an ammo count of 0 . Since you fired, you now have a
 * score of 1 and an ammo count of 0 .
 *
 * Sample Input 2
 * 9
 * RBFFFBRRF
 *
 * Sample Output 2
 * 3
 *
 * Explanation for Sample Output 2
 * A possible sequence of actions that obtains this score is RRBFFRFFB.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let _ = lines.next().unwrap().unwrap();
    let enemy_moves: String = lines.next().unwrap().unwrap();

    let (mut our_score, mut our_ammo) = (0, 0);
    let mut enemy_ammo = 0;

    for enemy_move in enemy_moves.chars() {
        match enemy_move {
            // If enemy is going to fire without ammo, we are reloading/firing
            'F' if enemy_ammo == 0 => fire_or_reload(&mut our_score, &mut our_ammo),
            // If enemy is really going to fire, we need to block to not lose score
            'F' => enemy_ammo -= 1,

            // If enemy is reloading, we are reloading/firing
            'R' => {
                fire_or_reload(&mut our_score, &mut our_ammo);
                enemy_ammo += 1;
            }

            // If enemy is blocking, we are reloading
            'B' => our_ammo += 1,

            _ => unreachable!(),
        }
    }

    println!("{our_score}");
}

fn fire_or_reload(our_score: &mut usize, our_ammo: &mut usize) {
    if *our_ammo > 0 {
        *our_score += 1;
        *our_ammo -= 1;
    } else {
        *our_ammo += 1;
    }
}
