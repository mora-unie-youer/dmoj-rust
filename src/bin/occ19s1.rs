use std::io::BufRead;

/**
 * OCC '19 S1 - Floor Planning
 *
 * Angie is buying a house!
 *
 * The floor plan of the house can be represented as two rectangles on the Cartesian plane: one from ( x_1 , y_1 ) to
 * ( x_2 , y_2 ) and one from ( x_3 , y_3 ) to ( x_4 , y_4 ).
 *
 * She wants to know how large her house is. Can you help her find out?
 *
 * Constraints
 * 1 ≤ x_1 , x_2 , x_3 , x_4 , y_1 , y_2 , y_3 , y_4 ≤ 10^3
 *
 * Input Specification
 * The first line contains the space separated integers x_1 , y_1 , x_2 , y_2 (the coordinates of the first rectangle).
 * The second line contains the space separated integers x_3 , y_3 , x_4 , y_4 (the coordinates of the second rectangle).
 *
 * Output Specification
 * Output one integer, the total area of the floor plan.
 *
 * Sample Input
 *   2 6 11 10
 *   14 14 8 5
 *
 * Sample Output
 *   78
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let coords: Vec<isize> = lines
        .take(2)
        .map(|line| line.unwrap())
        .flat_map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut first = [(coords[0], coords[1]), (coords[2], coords[3])];
    let mut second = [(coords[4], coords[5]), (coords[6], coords[7])];
    first.sort_unstable();
    second.sort_unstable();

    let [a1, a2] = first;
    let [b1, b2] = second;
    let (a, b) = ((a1, a2), (b1, b2));

    let a_area = get_rectangle_area(a);
    let b_area = get_rectangle_area(b);
    let intersection = intersection_area(a, b);
    let total_area = a_area + b_area - intersection;

    println!("{}", total_area);
}

type Point = (isize, isize);
type Rectangle = (Point, Point);

fn get_rectangle_area((p1, p2): Rectangle) -> isize {
    (p2.0 - p1.0).abs() * (p2.1 - p1.1).abs()
}

fn intersection_area((a1, a2): Rectangle, (b1, b2): Rectangle) -> isize {
    let p1x = (a1.0.min(a2.0)).max(b1.0.min(b2.0));
    let p1y = (a1.1.min(a2.1)).max(b1.1.min(b2.1));
    let p2x = (a1.0.max(a2.0)).min(b1.0.max(b2.0));
    let p2y = (a1.1.max(a2.1)).min(b1.1.max(b2.1));

    if p1x < p2x && p1y < p2y {
        get_rectangle_area(((p1x, p1y), (p2x, p2y)))
    } else {
        0
    }
}
