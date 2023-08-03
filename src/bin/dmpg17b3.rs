use std::io::BufRead;

/**
 * DMPG '17 B3 - Heroes
 *
 * Roger is addicted to the game Fire Emblem Heroes! His main hero is Hector, who has h_H health and deals d_H damage per
 * turn. Hector is up against a foe who deals d_F damage per turn, and has h_F health. However, Hector's special, Buckler,
 * activates every 4th turn and negates all damage done against him in that turn, as well as continues to deal the regular
 * amount of damage.
 *
 * Given N of these enemies, can you find out who will come out victorious if Hector attacks first, and how many turns it
 * will take?
 *
 * Note: assume the turn counter, as well as Hector's health, reset with each foe.
 *
 * Constraints
 *   Subtask 1 [60%]
 *   1 ≤ N ≤ 1 000
 *   1 ≤ h_H , d_H , h_F , d_F ≤ 100
 *   
 *   Subtask 2 [40%]
 *   1 ≤ N ≤ 10^6
 *   1 ≤ h_H , d_H , h_F , d_F ≤ 10^9
 *
 * Input Specification
 * Line 1 : Three space separated integers, N , h_H , and d_H .
 * Lines 2 … N + 1 : Two space separated integer, the h_F and d_F for each foe.
 *
 * Output Specification
 * N lines, of the format Win x if Hector wins in x turns, or Lose x if Hector loses in x turns.
 *
 * Sample Input
 *   4 12 5
 *   4 2
 *   999 999
 *   5 12
 *   20 3
 *
 * Sample Output
 *   Win 1
 *   Lose 1
 *   Win 1
 *   Win 4
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_line = first_line.split_whitespace().map(|v| v.parse().unwrap());

    let n = first_line.next().unwrap();
    let hector_health = first_line.next().unwrap();
    let hector_damage = first_line.next().unwrap();

    for line in lines.take(n) {
        let line = line.unwrap();
        let (foe_health, foe_damage) = line.split_once(' ').unwrap();
        let (foe_health, foe_damage): (usize, usize) =
            (foe_health.parse().unwrap(), foe_damage.parse().unwrap());

        let (hector_won, turns) = find_winner(hector_health, hector_damage, foe_health, foe_damage);
        if hector_won {
            println!("Win {turns}");
        } else {
            println!("Lose {turns}");
        }
    }
}

fn find_winner(
    hector_health: usize,
    hector_damage: usize,
    foe_health: usize,
    foe_damage: usize,
) -> (bool, usize) {
    if foe_health <= hector_damage {
        return (true, 1);
    } else if hector_health <= foe_damage {
        return (false, 1);
    }

    let foe_max_turns = (foe_health + hector_damage - 1) / hector_damage;
    let hector_max_turns_without_buckler = (hector_health + foe_damage - 1) / foe_damage;
    let buckler_activations = (hector_max_turns_without_buckler - 1) / 3;
    let hector_max_turns_with_buckler = hector_max_turns_without_buckler + buckler_activations;

    if hector_max_turns_with_buckler < foe_max_turns {
        (false, hector_max_turns_with_buckler)
    } else {
        (true, foe_max_turns)
    }
}
