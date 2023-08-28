use std::io::BufRead;

/**
 * WC '15 Contest 3 J1 - Battle Predictions
 * Woburn Challenge 2015-16 Round 3 - Junior Division
 *
 * What is a savior but an emblem of hope – but a monument for the people to look up to? Should he be anything that the
 * world needs him to be – or should he be none of it? What does a savior owe the world?
 *
 * The time has come for the man of steel to finally face these difficult questions. Meanwhile, feelings of fever, rage and
 * powerlessness are festering within the dark knight of Gotham. He can do nothing but watch as the planet praise an alien
 * capable of annihilating it. The peace of Gotham he fought so long to protect is now compromised by a godly figure that
 * has unworthily gained the world's reverence.
 *
 * To take care of this threat for good, the masked billionaire will have to elaborate a plan like no other he's ever
 * concocted. In particular, Batman plans to use Kryptonite-based weapons and a heavily-armored mech suit to defeat the most
 * powerful being on Earth. The success of the battle is hugely dependent on the strength/purity of the Kryptonite weapons
 * as well as the durability of the mech suit. He has aggregated the attack power provided by his weapons and the defense
 * power provided by his suit into two ratings A B and D B respectively. Being a master detective, Batman has also carefully
 * scrutinized Superman's strength and techniques from past battles. Through this, he has aggregated Superman's attack power
 * and defense power into two ratings A S and D S respectively. Each of these four ratings is a positive integer no greater
 * than 100 .
 *
 * Clearly, Batman will emerge victoriously if his attack power is strictly greater than Superman's defense power, and his
 * defense power is strictly greater than Superman's attack power. On the other hand, Superman will prevail if his attack
 * power is strictly greater than Batman's defense, and his defense is also strictly greater than Batman's attack. If
 * neither of these situations occur, then unfortunately it will be unclear who will win, so the outcome will be difficult
 * to predict.
 *
 * Batman is the type of hero who needs contingency plans for his contingency plans, so there's no way he'll be willing to
 * enter the fight with surprises. Can you help him predict the results of the upcoming battle or determine that it's
 * inconclusive so Batman knows he must further prepare?
 *
 * Input Specification
 * The first line will contain the two space-separated integers A B and D B .
 * The second line will contain the two space-separated integers A S and D S .
 *
 * Output Specification
 * Output on a single line the result of the showdown – Batman if Batman will win, Superman if Superman will win, or
 * Inconclusive otherwise.
 *
 * Sample Input 1
 *   12 31
 *   100 60
 *
 * Sample Output 1
 *   Superman
 *
 * Sample Input 2
 *   1 4
 *   2 3
 *
 * Sample Output 2
 *   Inconclusive
 *
 * Sample Input 3
 *   50 51
 *   50 50
 *
 * Sample Output 3
 *   Inconclusive
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let batman = lines.next().unwrap().unwrap();
    let superman = lines.next().unwrap().unwrap();

    let mut values = batman
        .split_ascii_whitespace()
        .chain(superman.split_ascii_whitespace())
        .map(|v| v.parse().unwrap());
    let batman_attack: usize = values.next().unwrap();
    let batman_defense = values.next().unwrap();
    let superman_attack = values.next().unwrap();
    let superman_defense = values.next().unwrap();

    if batman_attack > superman_defense && batman_defense > superman_attack {
        println!("Batman");
    } else if superman_attack > batman_defense && superman_defense > batman_attack {
        println!("Superman");
    } else {
        println!("Inconclusive");
    }
}
