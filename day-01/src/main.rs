

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_comparisons)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::convert::TryInto;
use std::char;

use text_io::scan;






fn main() {

    let cont = read_file("src/input.txt");
    // let cont = read_file("src/mininput.txt");

    let nums: Vec<i32> = cont
                        .split_whitespace()
                        .map(|l| l.parse::<i32>())
                        .flatten()
                        .collect();

    // PART ONE
    {
        let mut inc_count   = 0;
        let mut last_val    = nums[0];

        for v in nums.iter() {
            if ( last_val < *v ) {
                inc_count += 1;
            }
            last_val   = *v;
        }

        println!("part 1: {}", inc_count );
    }

    // PART TWO
    {
        let mut count = 0;
        let mut i     = 3;
        let length    = nums.len();

        while ( i < length ) {

            let n0 = nums[i-0];
            let n1 = nums[i-1];
            let n2 = nums[i-2];
            let n3 = nums[i-3];
            
            let sum0 = n0 + n1 + n2;
            let sum1 = n1 + n2 + n3;

            if ( sum0 > sum1) {
                count += 1;
            }

            i += 1;
        }

        println!("part 2: {}", count);
    }

}





/*
 *
 * ...boilerplate goes here...
 *
 */
fn read_file(path: &str) -> String
{
    // let path = "src/input.txt";
    let cont = match std::fs::read_to_string(path)
    {
        Err(why) => panic!(">>> couldn't read {}: {} <<<", path, why),
        Ok(read) => read,
    };
    cont
}

