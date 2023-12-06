use aho_corasick::{AhoCorasick, Match, PatternID};
use once_cell::sync::Lazy;
use regex::{Regex, RegexSet};

fn main() {
    // let file_path = &args[1];
    let mut total: u32 = 0;
    let lines = include_str!("./input");
    // Consumes the iterator, returns an (Optional) String
    for line in lines.lines() {
        println!("{}", line);
        total += get_number(line);
    }

    println!("{}", total);
}

fn string_to_num(val: &str) -> Option<u32> {
    match val {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        "0" => Some(0),
        _ => None,
    }
}

fn find_numbers(line: &str) -> (u32, u32) {
    // let patterns: [&str; 10] = [
    //     "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "[1-9]",
    // ];

    // // Compile a set matching any of our patterns.
    // let set = RegexSet::new(patterns).unwrap();
    // // Compile each pattern independently.
    // let regexes: Vec<_> = set
    //     .patterns()
    //     .iter()
    //     .map(|pat| Regex::new(pat).unwrap())
    //     .collect();

    // // Match against the whole set first and identify the individual
    // // matching patterns.
    // let matches = set
    //     .matches(line)
    //     .into_iter()
    //     // Dereference the match index to get the corresponding
    //     // compiled pattern.
    //     .map(|index| &regexes[index])
    //     // To get match locations or any other info, we then have to search the
    //     // exact same haystack again, using our separately-compiled pattern.
    //     .map(|re| re.find(line).unwrap())
    //     .;

    let ac = Lazy::new(|| {
        AhoCorasick::new(&[
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ])
        .unwrap()
    });
    let matches: Vec<u32> = ac
        .find_overlapping_iter(line)
        .map(|mat| mat.pattern().as_u32() % 9 + 1)
        .collect();

    println!("{:?}", matches);
    // static RE: Lazy<Regex> =
    //     Lazy::new(|| Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap());

    // let numbers: Vec<&str> = RE.find_iter(line).map(|m| m.as_str()).collect();
    // println!("{:?}", numbers);
    (
        matches.first().expect("No number on the left.").to_owned(),
        matches.last().expect("No number on the right.").to_owned(),
    )
}

fn get_number(line: &str) -> u32 {
    let (left, right) = find_numbers(line);
    let num = left * 10 + right;
    println!("{}", num);
    return num.into();
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tricky_case() {
        assert_eq!(
            find_numbers("1six15ninebgnzhtbmlxpnrqoneightfhp"),
            ("1", "eight")
        );
    }
}
