use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::BufRead,
};

/**
 * CCC '00 S3 - Surfing
 * Canadian Computing Competition: 2000 Stage 1, Junior #5, Senior #3
 *
 * Every web page is identified by a string of characters known as a URL (Uniform Resource Locator). Web pages are formatted
 * using HTML (Hypertext Markup Language). HTML has many codes, collectively known as markup, that allow the author to
 * specify the format of the pages as well as to specify links to other pages. For this problem, we are concerned only with
 * the markup that identifies links to other pages within a given page.
 *
 * A link within the page is denoted <A HREF="URL"> where URL is the URL of some other page. A user viewing a page
 * containing a link may click on the link to view the other page.
 *
 * You are to write a program that reads a number of web pages and their associated URLs. For each link on each page, you
 * must print the URL of the page containing the link, and the URL of the page referred to by the link. Following the last
 * page, you are then given several pairs of URLs. For each pair, you are to assume that you are viewing the page identified
 * by the first URL, and determine whether it is possible to click a sequence of links so as to view the page identified by
 * the second URL. If so, you should print Can surf from here to there. where here and there are the two URLs. If not you
 * should print Can't surf from here to there.
 *
 * Input Specification
 * The first line of input will contain an integer n ≤ 100 , the number of web pages. For each web page, there will be a
 * line containing its URL, followed by several lines containing the page. The URL will consist of up to 80 non-blank
 * printable characters and will not contain any quotation marks. The first line of the page will be <HTML> and the last
 * line will be </HTML>. Each page will contain up to 100 links in the format described above. Each link will be contained
 * within a single line of input. URLs in the link will be those of pages given in the input. The markup keywords A, HREF,
 * and HTML will appear only in uppercase.
 *
 * Following the n pages will be several pairs of lines giving URLs required by the problem as specified above. The last
 * line of input will be The End.
 *
 * Output Specification
 * For each pair, print the appropriate message given above.
 *
 * Sample Input
 * 3
 * http://ccc.uwaterloo.ca
 * <HTML> <TITLE>This is the CCC page</TITLE>
 * Hello there boys
 * and girls.  <B>Let's</B> try <A HREF="http://abc.def/ghi"> a
 * little
 * problem </A>
 * </HTML>
 * http://abc.def/ghi
 * <HTML> Now is the <TITLE>time</TITLE> for all good people to program.
 * <A HREF="http://www.www.www.com">hello</A><A HREF="http://xxx">bye</A>
 * </HTML>
 * http://www.www.www.com
 * <HTML>
 * <TITLE>Weird and Wonderful World</TITLE>
 * </HTML>
 * http://ccc.uwaterloo.ca
 * http://www.www.www.com
 * http://www.www.www.com
 * http://ccc.uwaterloo.ca
 * The End
 *
 * Sample Output
 * Link from http://ccc.uwaterloo.ca to http://abc.def/ghi
 * Link from http://abc.def/ghi to http://www.www.www.com
 * Link from http://abc.def/ghi to http://xxx
 * Can surf from http://ccc.uwaterloo.ca to http://www.www.www.com.
 * Can't surf from http://www.www.www.com to http://ccc.uwaterloo.ca.
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut urls: HashMap<String, HashSet<String>> = HashMap::new();

    while urls.len() != n {
        let origin = lines.next().unwrap().unwrap();

        let mut content = String::new();
        for line in lines.by_ref() {
            let line = line.unwrap();
            content.push_str(&line);
            if line.ends_with("</HTML>") {
                break;
            }
        }

        let mut links = HashSet::new();
        for chunk in content.split_ascii_whitespace() {
            if chunk.starts_with("HREF=") {
                let to = chunk.split('"').nth(1).unwrap();
                links.insert(to.to_owned());
                println!("Link from {origin} to {to}");
            }
        }

        urls.insert(origin, links);
    }

    while let Some(origin) = lines.next() {
        let origin = origin.unwrap();
        if origin == "The End" {
            break;
        }

        let to = lines.next().unwrap().unwrap();
        let can_surf = can_surf_to(&origin, &to, &urls);

        if can_surf {
            println!("Can surf from {origin} to {to}.");
        } else {
            println!("Can't surf from {origin} to {to}.");
        }
    }
}

fn can_surf_to(origin: &str, to: &str, urls: &HashMap<String, HashSet<String>>) -> bool {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(origin);
    queue.push_front(origin);

    while let Some(site) = queue.pop_front() {
        if site == to {
            return true;
        }

        let links = match urls.get(site) {
            Some(links) => links,
            None => continue,
        };

        for link in links {
            if visited.insert(link) {
                queue.push_back(link);
            }
        }
    }

    false
}
