use std::io::BufRead;

/**
 * DMOPC '19 Contest 5 P2 - Charlie's Crazy Conquest
 *
 * Charlie just downloaded the new free-to-play game Raid: Shadow Legends Conquest. Just what he wanted! In this game, he
 * must defeat enemies, all of which are bots, and restore power to his clan. Both Charlie and the enemy before him start
 * with H health points. Then, the two take turns performing actions, with Charlie going first. On each turn, one can
 * perform one of two actions:
 *   A d Attack your opponent, dealing d damage.
 *   D d Dodge your opponent if they attack on the next turn. If they do not attack on the next turn, take d damage from
 * self-humility.
 *
 * Because he is such a computer genius, Charlie has hacked the game and created two lists of N actions each representing
 * what the opponent will do and what he will do. Your job is to simulate his battle and find out who wins. If any person's
 * health reaches 0 or below, your program is to output the correct answer and terminate.
 *
 * Note: Dodging at the end of the list of actions counts as a failed dodge. (i.e. if the enemy prepares a dodge as their
 * last move, they will inflict self-harm.)
 *
 * Input Specification
 * The first line of input contains two space separated positive integers N and H .
 *
 * The next N lines contain an uppercase Latin letter and a non-negative integer d representing Charlie's actions.
 *
 * The next N lines contain an uppercase Latin letter and a non-negative integer d representing his opponent's actions.
 *
 * Output Specification
 * Output VICTORY if Charlie wins or DEFEAT if he loses or TIE if none of them die.
 *
 * Constraints
 * 1 ≤ N , H ≤ 1000
 * 0 ≤ d ≤ 1000
 *
 * Sample Input 1
 * 3 100
 * A 50
 * D 10
 * A 100
 * A 90
 * D 0
 * A 0
 *
 * Sample Output 1
 * DEFEAT
 *
 * Sample Explanation 1
 * After the first turn, the bot's health is at 50. After the second turn, Charlie's health is at 10. After the third turn,
 * Charlie attempts to dodge the next attack but his opponent also dodges causing Charlie's health to drop to 0. The
 * remaining commands are ignored due to Charlie losing.
 *
 * Sample Input 2
 * 4 100
 * D 10
 * D 20
 * D 30
 * D 30
 * D 10
 * D 20
 * D 30
 * D 40
 *
 * Sample Output 2
 * VICTORY
 *
 * Sample Explanation 2
 * After the second last turn, Charlie's health is at 10 and his opponent's is at 40. Then, his opponent tries to dodge but
 * Charlie does not attack (because the list of actions has been completed) and his opponent takes 40 damage reducing their
 * health to 0.
 *
 * Sample Input 3
 * 2 100
 * A 99
 * A 99
 * D 1
 * A 1
 *
 * Sample Output 3
 * TIE
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (n, h) = first_line.split_once(' ').unwrap();
    let (n, h): (usize, usize) = (n.parse().unwrap(), h.parse().unwrap());

    let charlie_moves: Vec<(u8, usize)> = lines
        .by_ref()
        .take(n)
        .map(Result::unwrap)
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(a, b)| (a.as_bytes()[0], b.parse().unwrap()))
        })
        .collect();
    let enemy_moves: Vec<(u8, usize)> = lines
        .by_ref()
        .take(n)
        .map(Result::unwrap)
        .filter_map(|line| {
            line.split_once(' ')
                .map(|(a, b)| (a.as_bytes()[0], b.parse().unwrap()))
        })
        .collect();

    let all_moves: Vec<(u8, usize)> = charlie_moves
        .into_iter()
        .zip(enemy_moves)
        .flat_map(|(a, b)| [a, b])
        .collect();

    let mut current_player = h;
    let mut another_player = h;
    for i in 0..all_moves.len() {
        let (action, damage) = all_moves[i];

        if action == b'A' && (i == 0 || all_moves[i - 1].0 != b'D') {
            another_player = another_player.saturating_sub(damage);
        } else if action == b'D' && (i == all_moves.len() - 1 || all_moves[i + 1].0 != b'A') {
            current_player = current_player.saturating_sub(damage);
        }

        if current_player == 0 {
            if i % 2 == 0 {
                println!("DEFEAT");
            } else {
                println!("VICTORY");
            }

            return;
        } else if another_player == 0 {
            if i % 2 == 0 {
                println!("VICTORY");
            } else {
                println!("DEFEAT");
            }

            return;
        }

        std::mem::swap(&mut current_player, &mut another_player);
    }

    if current_player != 0 && another_player != 0 {
        println!("TIE");
        return;
    }

    unreachable!()
}
