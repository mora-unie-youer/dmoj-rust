use std::{collections::HashSet, io::BufRead};

/**
 * New Year's '16 P1 - Late Christmas Shopping
 *
 * Did you know that eagles hibernate? At least, one eagle does. When the winter break started, FatalEagle decided that he
 * would retreat into his mancave for a little while to hibernate and take a break from all the hustle-bustle of school
 * life. But when he finally came out, he discovered that he had missed Christmas and it was already the New Year! What a
 * disaster! If he doesn't get the other DMOJ admins their presents, they will definitely refuse when he asks them to be his
 * data slaves!
 *
 * Luckily, there's a special store where the boxing week sales are still going on so FatalEagle can still get some cheap
 * presents. FatalEagle brought N shopping bags with him, one for each DMOJ admin's presents. FatalEagle is known for being
 * a calm, cool, and collected shopper normally. When browsing the aisles of the store however, he found that their prices
 * were unbelievably low! Amazed at his luck, FatalEagle went on a shopping spree, grabbing all that he could and shoving
 * them into his shopping bags indiscriminately.
 *
 * It was only when he lined up for the checkout that he realized he may have made a mistake. What if two or more DMOJ
 * admins end up getting the same present? FatalEagle can't let his reputation as a great gifter be soiled like that! As the
 * nice cashier of the store, you have agreed to help FatalEagle check if multiple bags contain the same item.
 *
 * Input Specification
 * The first line of input will contain N ( 1 ≤ N ≤ 3 ) , the number of shopping bags.
 * Each of the next N lines will contain M ( 1 ≤ M ≤ 3 ) , the number of items in that bag, followed by M integers, the
 * label of each item. The labels of the items will be between 1 and 10 , inclusive.
 *
 * Output Specification
 * Output YES if there's at least one gift that multiple admins will get, otherwise output NO.
 *
 * Sample Input 1
 *   2
 *   3 1 2 3
 *   3 2 4 6
 *
 * Sample Output 1
 *   YES
 *
 * Explanation for Sample Output 1
 *   Both admins will get item 2.
 *
 * Sample Input 2
 *   2
 *   3 1 3 5
 *   3 2 4 6
 *
 * Sample Output 2
 *   NO
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut all_presents = HashSet::new();
    for line in lines.take(n) {
        let line = line.unwrap();
        let mut parts = line.split_ascii_whitespace().map(|v| v.parse().unwrap());
        let count: usize = parts.next().unwrap();
        let presents: HashSet<usize> = parts.take(count).collect();

        if all_presents.intersection(&presents).count() > 0 {
            println!("YES");
            return;
        } else {
            all_presents = all_presents.union(&presents).copied().collect();
        }
    }

    println!("NO");
}
