use std::fs::File;
use std::io::prelude::*;
use substring::Substring;

fn main() -> std::io::Result<()> {
    let mut file = File::open("testcase/example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // How the data is extrapolated into usable strings.
    // let sum = Solution::ValidSum(&contents);
    assert_eq!("foobar".to_string().substring(1, 6), "oobar");
    //println!("{}", sum);

    Ok(())
}
/*
struct Solution;

impl Solution {
    fn ValidSum(contents: &String) -> i32 {
        let r = 'red'
        let g = 'green'
        let b = 'blue'
    
        print('Substring count =', s.count('a'))
    
        s = 'This Is The Best Theorem'
        print('Substring count =', s.count('Th'))
        let mut sum = 0;
        let lines = contents.lines();
        // 12 red cubes
        // 13 green cubes
        // 14 blue cubes
        assert_eq!("foobar".to_string().substring(1,6), "oobar");
        /*
        for line in lines {
            println!("{}", line);
            //
            if {
                // remove
                continue;
            } else {
                // add game number to sum
            }
        }

        return sum
        */
    }
}
*/
