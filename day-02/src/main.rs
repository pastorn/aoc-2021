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




struct SubmarinePos {
    x: i32,
    y: i32,
}

/*
let coin = "head";
    match coin{
        "head" => println!("head"),
        "tail" => println!("tail"),
        _      => println!("False coin"),
    };
*/

fn split_for(inp_str: &str, c : char) -> Option<(&str, &str)> {
    let mut iter = inp_str.split(c);
    let o1 = iter.next()?;
    let o2 = iter.next()?;
    Some( (o1, o2) )
}

fn main() {

    let cont = read_file("src/input.txt");
    // let cont = read_file("src/mininput.txt");

    let nums: Vec<String> = cont
                        .split_whitespace()
                        .map(|l| l.parse::<String>())
                        .flatten()
                        .collect();

    // PART ONE
    {
        let mut pos = SubmarinePos { x: 0, y: 0, };
        
        
        // for l in 
        // println!("part 1: {}", inc_count );
    }

    // PART TWO
    {

        // println!("part 2: {}", count);
    }

}





/*
 * ...boilerplate goes here...
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

