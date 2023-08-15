use std::io::BufRead;

/**
 * CCC '08 S1 - It's Cold Here!
 * Canadian Computing Competition: 2008 Stage 1, Senior #1
 *
 * Canada is cold in winter, but some parts are colder than others. Your task is very simple, you need to find the coldest
 * city in Canada. So, when given a list of cities and their temperatures, you are to determine which city in the list has
 * the lowest temperature and is thus the coldest.
 *
 * Input
 * The input is a sequence of city names and temperature values. Temperatures are integer, possibly preceded with a "minus"
 * sign. There is a single space between the city name and the temperature. No city name contains any whitespace and is
 * always less than 256 characters in length. There is at least one city in the list, no more than 10 000 cities, and the
 * last city is always Waterloo. You may assume that the temperature is not less than − 273 and not more than 200 .
 *
 * Output
 * You are to output the name of the coldest city on a single line with no whitespace before or after the name. You may
 * assume that there will not be more than one city which is the coldest.
 *
 * Sample Input
 * Saskatoon -20
 * Toronto -2
 * Winnipeg -40
 * Vancouver 8
 * Halifax 0
 * Montreal -4
 * Waterloo -3
 *
 * Output for Sample Input
 * Winnipeg
 */
fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let (mut coldest, mut min_temp) = (String::new(), std::isize::MAX);
    for line in lines {
        let line = line.unwrap();
        let (name, temp) = line.split_once(' ').unwrap();
        let temp = temp.parse().unwrap();

        if temp < min_temp {
            coldest = name.to_owned();
            min_temp = temp;
        }
    }

    println!("{coldest}");
}
