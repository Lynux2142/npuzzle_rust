mod parsing;
mod map;
mod map_procedure;

use map_procedure::core_swap;
use map::*;
use parsing::*;

use std::env;
use std::fs::File;
use std::error::Error;

fn      main()
{
    let args: Vec<String> = env::args().collect();
    let mut map: Map = Map::new();
    let file = match File::open(&args[1])
    {
        Ok(file) => file,
        Err(e) => panic!("error: {}", e.description())
    };

    parse(&mut map, file);


    core_swap(&map, 'd');
    println!("test : {:?}", map);
    /*for i in 0..map.size
    {
        print!("{} ", map.grid[i]);
    }
    println!();*/
}
