use std::env;

fn main() {
    let source = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let mut result: Vec<String> = vec![];

    let src_iter = source.split_whitespace();
    let mut src_reverse_iter = src_iter.clone().rev().map(|word| word.chars().next());

    for word in src_iter {
        let mut result_word = String::with_capacity(word.len());

        match src_reverse_iter.next().unwrap() {
            Some(new_first_char) => result_word.push(new_first_char),
            None => continue,
        };

        if let Some((position, _)) = word.char_indices().nth(1) {
            result_word.push_str(&word[position..]);
        }

        result.push(result_word);
    }

    println!("{}", result.join(" "));
}
