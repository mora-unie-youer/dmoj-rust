use std::io::BufRead;

/**
 * DMOPC '14 Exam Time P1 - French Homework
 *
 * One day you are too busy figuring out a programming contest question to realize that you have French homework due the
 * next day!
 *
 * Surprisingly, your French teacher decides to mark the sheets and sees you rapidly scribbling as she passes by. Since she
 * knows that you are very talented at programming, she decides to give you another chance — with a catch.
 *
 * The second chance is that you may finish completing the worksheet in class, however; you must directly copy all the
 * answers from a program you wrote under her direct supervision.
 *
 * In French, the order of sentences generally follows a Subject-Verb-Object structure. For questions, this can change to a
 * Verb-Subject-Object order. We will assume that the subject will always be tu and note that the verb will already be
 * conjugated in the present tense for the second person singular.
 *
 * In addition, you notice the following pattern (which is true most of the time):
 *   If the word ends with e, then it is feminine (la)
 *   If the word ends with s, then it is plural (les)
 *   Otherwise, we can assume the word to be masculine (le)
 *
 * You also notice on the sheet that none of the words on the second line of each case begin with a vowel.
 *
 * Input Specification
 * The first line of input will consist of the verb while the second line of input will consist of the object. Each word will be up to 30 characters in length.
 *
 * Output Specification
 * Output the words as a question formed using inversion.
 *
 * Sample Input 1
 *   Fais
 *   devoirs
 *
 * Sample Output 1
 *   Fais-tu les devoirs ?
 *
 * Sample Input 2
 *   Aimes
 *   ordinateurs
 *
 * Sample Output 2
 *   Aimes-tu les ordinateurs ?
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let verb = lines.next().unwrap().unwrap();
    let object = lines.next().unwrap().unwrap();

    let article = match object.chars().rev().next() {
        Some('e') => "la",
        Some('s') => "les",
        Some(_) => "le",
        None => unreachable!(),
    };

    println!("{verb}-tu {article} {object} ?");
}
