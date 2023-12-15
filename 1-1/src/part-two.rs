use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("testcase/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let sum = Solution::evaluation(&contents);

    println!("{}", sum);

    Ok(())
}

struct Solution;

impl Solution {
    fn evaluation(contents: &String) -> i32 {
        // not using regex, too much syntax/mess.
        let mut sum: i32 = 0;
        let re = Regex::new(r"[0-9]|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
        //its so tiresome monkey autism
        for line in contents.lines() {
            let frun = re.captures(line).unwrap();
            if frun.len() == 0 {
                match frun.get(0).map(|m| m.as_str()) {
                    Some("0" | "zero") => sum += 0,
                    Some("1" | "one") => sum += 10,
                    Some("2" | "two") => sum += 20,
                    Some("3" | "three") => sum += 30,
                    Some("4" | "four") => sum += 40,
                    Some("5" | "five") => sum += 50,
                    Some("6" | "six") => sum += 60,
                    Some("7" | "seven") => sum += 70,
                    Some("8" | "eight") => sum += 80,
                    Some("9" | "nine") => sum += 90,
                    _ => continue,
                }
                match frun.get(0).map(|m| m.as_str()) {
                    Some("0" | "zero") => sum += 0,
                    Some("1" | "one") => sum += 1,
                    Some("2" | "two") => sum += 2,
                    Some("3" | "three") => sum += 3,
                    Some("4" | "four") => sum += 4,
                    Some("5" | "five") => sum += 5,
                    Some("6" | "six") => sum += 6,
                    Some("7" | "seven") => sum += 7,
                    Some("8" | "eight") => sum += 8,
                    Some("9" | "nine") => sum += 9,
                    _ => continue,
                }
            } else {
                let mut is_match: bool = false;
                match frun.get(0).map(|m| m.as_str()) {
                    Some("0" | "zero") => {
                        sum += 0;
                        is_match = true;
                    }
                    Some("1" | "one") => {
                        sum += 10;
                        is_match = true;
                    }
                    Some("2" | "two") => {
                        sum += 20;
                        is_match = true;
                    }
                    Some("3" | "three") => {
                        sum += 30;
                        is_match = true;
                    }
                    Some("4" | "four") => {
                        sum += 40;
                        is_match = true;
                    }
                    Some("5" | "five") => {
                        sum += 50;
                        is_match = true;
                    }
                    Some("6" | "six") => {
                        sum += 60;
                        is_match = true;
                    }
                    Some("7" | "seven") => {
                        sum += 70;
                        is_match = true;
                    }
                    Some("8" | "eight") => {
                        sum += 80;
                        is_match = true;
                    }
                    Some("9" | "nine") => {
                        sum += 90;
                        is_match = true;
                    }
                    _ => continue,
                }
                if is_match {
                    let er = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez")
                        .unwrap();
                    let reverse: String = line.chars().rev().collect();
                    let nruf = er.captures(&reverse).unwrap();
                    println!("{}", reverse);
                    match nruf.get(0).map(|m| m.as_str()) {
                        Some("0" | "orez") => sum += 0,
                        Some("1" | "eno") => sum += 1,
                        Some("2" | "owt") => sum += 2,
                        Some("3" | "eerht") => sum += 3,
                        Some("4" | "ruof") => sum += 4,
                        Some("5" | "evif") => sum += 5,
                        Some("6" | "xis") => sum += 6,
                        Some("7" | "neves") => sum += 7,
                        Some("8" | "thgie") => sum += 8,
                        Some("9" | "enin") => sum += 9,
                        _ => continue,
                    }
                }
            }
        }
        return sum;
    }
}
