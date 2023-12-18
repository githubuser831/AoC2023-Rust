use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("testcase/example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // How the data is extrapolated into usable strings.
    let sum = Solution::ValidSum(&contents);
    
    println!("{}", sum);
  
    Ok(())
}

struct Solution;

impl Solution {
    fn ValidSum(contents: &String) -> i32 {
        let mut sum = 0;
        let lines = contents.lines();

        for line in lines {
            println!("{}", line);
            if {
                // check for invalids
                continue;
            }else {
                // add game number to sum
            }
        }
        
        return sum
        
    }
}
