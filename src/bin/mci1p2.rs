use std::io::BufRead;

/**
 * MCIPC Contest 1 P2 - Infinite Chessboard
 *
 * Peter is playing chess. However, he isn't playing any ordinary game of chess, he is playing infinite chess! In infinite
 * chess the board is just like a normal chessboard, only infinite.
 *
 * Each square has its own coordinate which is defined by R and C , where R is how many rows down it is from the top, and C
 * is how many columns it is from the left.
 *
 * Furthermore, each square is either black or white. The top left square at row 1 column 1 has a colour of white. In a
 * chessboard (and an infinite chessboard) the board's colours are ordered in a checkered pattern, that is a white square
 * will only neighbour black squares, and a black square will only neighbour white squares.
 *
 * Peter has a piece at a certain position, he wants to know if that piece is on a white square or a black square. Can you
 * help him?
 *
 * Constraints
 *   1 ≤ R , C ≤ 10 18
 *
 * Input Specification
 * The first and only line of input will contain two integers R and C separated by a space.
 *
 * Output Specification
 * Output either black or white depending on if the square the piece is on is black or white.
 *
 * Sample Input 1
 *   1 1
 *
 * Sample Output 1
 *   white
 *
 * Sample Input 2
 *   3 4
 *
 * Sample Output 2
 *   black
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let (row, col) = line.split_once(' ').unwrap();
    let (row, col): (usize, usize) = (row.parse().unwrap(), col.parse().unwrap());

    // Making row and col start from 0
    let (row, col) = (row - 1, col - 1);

    // Determining color
    if (row + col) % 2 == 0 {
        println!("white");
    } else {
        println!("black");
    }
}
