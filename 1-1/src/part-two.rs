use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let mut file = File::open("testcase/example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let calibrated: Vec<i64> = Solution::read_value(&contents);
    let calibrated_sum = Solution::calibrate_value(calibrated);
    println!("Calibration Sum: {}", calibrated_sum);
    Ok(())
}

struct Solution;

impl Solution {
  pub fn read_value(contents: &str) -> Vec<i64> {
      let mut calibrated: Vec<i64> = Vec::new();
      let re = Regex::new("[0-9]").unwrap();
      let lines = contents.lines();
      for line in lines {
        let cleanup: String = re.replace_all(line, "").to_string();
        let ne = Regex::new("[a-z]").unwrap();
        let zero = Regex::new("zero").unwrap();
        let one = Regex::new("one").unwrap();
        let two = Regex::new("two").unwrap();
        let three = Regex::new("three").unwrap();
        let four = Regex::new("four").unwrap();
        let five = Regex::new("five").unwrap();
        let six = Regex::new("six").unwrap();
        let seven = Regex::new("seven").unwrap();
        let eight = Regex::new("eight").unwrap();
        let nine = Regex::new("nine").unwrap();
        let mut translate: String = cleanup.clone();
        translate = zero.replace_all(&translate, "0").to_string();
        translate = one.replace_all(&translate, "1").to_string();
        translate = two.replace_all(&translate, "2").to_string();
        translate = three.replace_all(&translate, "3").to_string();
        translate = four.replace_all(&translate, "4").to_string();
        translate = five.replace_all(&translate, "5").to_string();
        translate = six.replace_all(&translate, "6").to_string();
        translate = seven.replace_all(&translate, "7").to_string();
        translate = eight.replace_all(&translate, "8").to_string();
        translate = nine.replace_all(&translate, "9").to_string();
        let mut uncalibrated: String = ne.replace_all(&translate, "").to_string();
        print!("uncalibrated{}", uncalibrated);
        let oe = Regex::new("\\d").unwrap();
        
        if uncalibrated.len() == 1 {
          if let Some(duplicate_captures) = oe.captures(&uncalibrated) {
            if let Some(duplicate) = duplicate_captures.get(1).map(|m| m.as_str()) {
              uncalibrated = format!("{}{}", duplicate, duplicate);
            } else {
            }
          }
        } else if uncalibrated.len() >= 2 {
  
          let first = uncalibrated.chars().nth(0).unwrap().to_string();

          let last = uncalibrated.chars().rev().nth(0).unwrap().to_string();

          uncalibrated = format!("{}{}", first, last);
        }
        if let Ok(parsed) = uncalibrated.parse::<i64>() {
          calibrated.push(parsed);
        } else{
        }
      }
    calibrated
  }
  pub fn calibrate_value(calibrated: Vec<i64>) -> i64 {
    print!("{:?}", calibrated);
    let sum: i64 = calibrated.iter().sum();
    sum
  }
}
