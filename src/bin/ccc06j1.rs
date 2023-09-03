use std::io::BufRead;

/**
 * CCC '06 J1 - Canadian Calorie Counting
 * Canadian Computing Competition: 2006 Stage 1, Junior #1
 *
 * At Chip's Fast Food emporium there is a very simple menu. Each food item is selected by entering a digit choice.
 * Here are the three burger choices:
 *   1 – Cheeseburger (461 Calories)
 *   2 – Fish Burger (431 Calories)
 *   3 – Veggie Burger (420 Calories)
 *   4 – no burger
 *
 * Here are the three drink choices:
 *   1 – Soft Drink (130 Calories)
 *   2 – Orange Juice (160 Calories)
 *   3 – Milk (118 Calories)
 *   4 – no drink
 *
 * Here are the three side order choices:
 *   1 – Fries (100 Calories)
 *   2 – Baked Potato (57 Calories)
 *   3 – Chef Salad (70 Calories)
 *   4 – no side order
 *
 * Here are the three dessert choices:
 *   1 – Apple Pie (167 Calories)
 *   2 – Sundae (266 Calories)
 *   3 – Fruit Cup (75 Calories)
 *   4 – no dessert
 *
 * Write a program that will compute the total Calories of a meal.
 *
 * Input Specification
 * The program should input a number for each type of item then calculate and display the Calorie total. The first line will
 * be the customer's choice of burger, the second will be the choice of side, then drink, then dessert. You may assume that
 * each input will be a number from 1 to 4. That is, each customer has to pick exactly one number from each of the four
 * options out of each of the four categories.
 *
 * Output Specification
 * The program prints out the total Calories of the selected meal, and stops executing after this output.
 *
 * Sample Input
 * 2
 * 1
 * 3
 * 4
 *
 * Sample Output
 * Your total Calorie count is 649.
 *
 * Explanation
 * The customer chose Burger #2, Side #1, Drink #3 and Dessert #4.
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let mut params = lines
        .take(4)
        .map(Result::unwrap)
        .map(|v| v.parse::<usize>().unwrap());

    let mut calories = 0;

    // Burger
    calories += match params.next().unwrap() {
        1 => 461,
        2 => 431,
        3 => 420,
        4 => 0,
        _ => unreachable!(),
    };

    // Side
    calories += match params.next().unwrap() {
        1 => 100,
        2 => 57,
        3 => 70,
        4 => 0,
        _ => unreachable!(),
    };

    // Drink
    calories += match params.next().unwrap() {
        1 => 130,
        2 => 160,
        3 => 118,
        4 => 0,
        _ => unreachable!(),
    };

    // Dessert
    calories += match params.next().unwrap() {
        1 => 167,
        2 => 266,
        3 => 75,
        4 => 0,
        _ => unreachable!(),
    };

    println!("Your total Calorie count is {calories}.");
}
