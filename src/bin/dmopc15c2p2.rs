use std::{cmp::Ordering, io::BufRead};

/**
 * DMOPC '15 Contest 2 P2 - Admin War
 *
 * Xyene and FatalEagle don't always get along. When they're feeling particularly confrontational, they usually quietly
 * settle the matter with a friendly game of Admin War.
 *
 * In this two-person game, the players are each dealt N cards. The players will then continuously draw cards from the top
 * of their decks and compare their values, with the player whose card has a higher value winning 1 point. In the case of a
 * tie, neither player wins a point. All cards are discarded after being played and the game ends when neither player has
 * any more cards.
 *
 * The player with more points when the game ends is the winner. If both players have the same amount of points, the game
 * results in a tie.
 *
 * The normal cards in a deck of cards are, from lowest to highest value: 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , J , Q , K , A
 * However, Xyene and FatalEagle are too busy to buy an actual deck, and instead play with a sketchy special deck
 * where J , Q , K , and A are replaced with 11 , 12 , 13 , and 14 , respectively. Their deck can also contain more than 4
 * copies of each card.
 *
 * You have taken a peek at each of their decks while they weren't paying attention and would like to know the outcome of
 * the game.
 *
 * Input Specification
 * The first line of input contains N ( 1 ≤ N ≤ 200 ) , the number of cards in each player's deck.
 *
 * The second line contains N space-separated integers, indicating Xyene's cards from top to bottom.
 *
 * The third line contains N space-separated integers, indicating FatalEagle's cards from top to bottom.
 *
 * Card values are guaranteed to be between 2 and 14 , inclusive.
 *
 * Output Specification
 * On the first line, output Xyene and FatalEagle's final scores respectively, separated by a space.
 *
 * On the second line, output the winner of the game: Xyene, FatalEagle, or Tie if there is a tie.
 *
 * Sample Input 1
 * 3
 * 7 13 4
 * 2 14 11
 *
 * Sample Output 1
 * 1 2
 * FatalEagle
 *
 * Explanation for Sample Output 1
 * Xyene won the first round but FatalEagle won the next two. Therefore, the final score is 1 : 2 , and FatalEagle is the
 * winner.
 *
 * Sample Input 2
 * 5
 * 14 14 14 14 14
 * 2 2 2 2 2
 *
 * Sample Output 2
 * 5 0
 * Xyene
 *
 * Explanation for Sample Output 2
 * Xyene sweeped won all rounds by a wide margin, winning the game with a final score of 5 : 0 .
 *
 * Sample Input 3
 * 6
 * 5 12 6 14 7 9
 * 10 12 6 13 8 8
 *
 * Sample Output 3
 * 2 2
 * Tie
 *
 * Explanation for Sample Output 3
 * Xyene won rounds 4 and 6 , FatalEagle won rounds 1 and 5 , and rounds 2 and 3 culminated in ties. Therefore, the final
 * score is 2 : 2 and the game ends in a tie.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let xyene = lines.next().unwrap().unwrap();
    let fataleagle = lines.next().unwrap().unwrap();

    let xyene = xyene
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse::<usize>().unwrap());
    let fataleagle = fataleagle
        .split_ascii_whitespace()
        .take(n)
        .map(|v| v.parse::<usize>().unwrap());

    let (xyene_score, fataleagle_score) = xyene.zip(fataleagle).fold((0, 0), |(xs, fs), (x, f)| {
        (xs + (x > f) as usize, fs + (x < f) as usize)
    });

    println!("{xyene_score} {fataleagle_score}");
    match xyene_score.cmp(&fataleagle_score) {
        Ordering::Less => println!("FatalEagle"),
        Ordering::Greater => println!("Xyene"),
        Ordering::Equal => println!("Tie"),
    }
}
