mod parsing;
mod map;

use std::env;
use std::fs::File;
use std::error::Error;
use parsing::*;
use map::*;

fn      main()
{
    let args: Vec<String> = env::args().collect();
    let map: Map;
    let file = match File::open(&args[1])
    {
        Ok(file) => file,
        Err(e) => panic!("error: {}", e.description())
    };

    map = parse(file);
    for i in 0..map.size
    {
        print!("{} ", map.grid[i]);
    }
    println!();
}
