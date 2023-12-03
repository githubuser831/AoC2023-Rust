use std::fs::File;
use regex::Regex;
use std::io::Read;
  
pub fn main() -> std::io::Result<()> {
    let mut file = File::open("testcase/example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    
    let output = Solution::read_line(self, &contents)

    println!("Output: {}", output);

    Ok(())
}

// Above is how testcases are read, calls function, then returns output.

struct Solution;

impl Solution{
    pub fn read_line(&mut self, contents: &mut str) -> String {
      let re = Regex::new("[a-z]").unwrap();
      contents = re.replace_all(contents, "").to_string();
    }
    /*pub fn calibrate_value{
    let mut array = [];
    
    }
    pub fn calibration_sum() -> i8 {
      let sum = a.iter().sum();
      return sum;
    }*/

}
