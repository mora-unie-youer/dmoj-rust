use std::io::BufRead;

/**
 * The Cosmic Era P1 - Ship
 *
 * The Archangel is being built! In order to build the Archangel, at least 1 of a number of parts are required. In the
 * following table, each part is represented by an uppercase character.
 * Part Name           Letter Used
 * Beam Weapons        B
 * Frame (inclusive)   F
 * Thrusters           T
 * Launch Pad          L
 * Command Room        C
 *
 * However, you appear to be missing some parts. Can you figure out which?
 *
 * Input Specification
 * The first line contains a string containing the identifiers of all the parts you have. The length of the string will be
 * at least 1 and no longer than 10 .
 *
 * Output Specification
 * The missing parts, each on a separate line and in any order. If there are no missing parts, output NO MISSING PARTS.
 *
 * Sample Input 1
 *   BFTL
 *
 * Sample Output 1
 *   C
 *
 * Sample Input 2
 *   BFTLC
 *
 * Sample Output 2
 *   NO MISSING PARTS
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    const PARTS: [char; 5] = ['B', 'F', 'T', 'L', 'C'];
    let parts = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|ch| PARTS.into_iter().position(|c| c == ch).unwrap())
        .fold(0, |acc, i| acc | (1 << i));

    if parts == (1 << 5) - 1 {
        println!("NO MISSING PARTS");
    } else {
        for (i, part) in PARTS.into_iter().enumerate() {
            let mask = 1 << i;
            if parts & mask == 0 {
                println!("{}", part);
            }
        }
    }
}
