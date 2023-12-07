use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut file = File::open("testcase/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // How the data is extrapolated into usable strings.
    println!("{}", &contents);
    let calibrated: Vec<i64> = Solution::read_value(&contents);
    let calibrated_sum = Solution::calibrate_value(calibrated);
    println!("Calibration Sum: {}", calibrated_sum);
    Ok(())
}


struct Solution;

impl Solution {
  pub fn read_value(contents: &str) -> Vec<i64> {
      let mut calibrated: Vec<i64> = Vec::new();
      let re = Regex::new("[a-z]").unwrap();
      let oe = Regex::new("(\\d)").unwrap();
      let lines = contents.lines();

      for line in lines {
        let mut uncalibrated = re.replace_all(line, "").to_string();
        if uncalibrated.len() < 2 {
          if let Some(duplicate_captures) = oe.captures(&uncalibrated) {
          let duplicate = duplicate_captures.get(1).unwrap().as_str();
          uncalibrated = format!("{}{}", duplicate, duplicate)
          }
        } else {
  
          let first = uncalibrated.chars().nth(0).unwrap().to_string();

          let last = uncalibrated.chars().rev().nth(0).unwrap().to_string();

          uncalibrated = format!("{}{}", first, last);
        }
        if let Ok(parsed) = uncalibrated.parse::<i64>() {
          calibrated.push(parsed);
        } else{
          eprintln!("Parsing Error: {}", uncalibrated);
        }
      }
    calibrated
  }
  pub fn calibrate_value(calibrated: Vec<i64>) -> i64 {
    println!("{:?}", calibrated);
    let sum: i64 = calibrated.iter().sum();
    sum
  }
}
