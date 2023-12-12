use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("testcase/example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Solution::sanitize(&contents);
    Solution::evaluate(&contents);
    Ok(())
}

//How text is handled into strings

struct Solution;

impl Solution {
    fn sanitize(contents: &String) -> String {
        let sa = Regex::new("[0-9]").unwrap();
        let nu = sa.replace_all(&contents, "");
        print!("{}", nu);
        // meant to sanitize only numbers
    }
    fn evaluate(contents: &String) -> String {
        let ab = Regex::new("zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let reverse: String = contents.chars().rev().collect();
        let ba = Regex::new("orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|").unwrap();
        print!("{}", reverse);
    }
}
