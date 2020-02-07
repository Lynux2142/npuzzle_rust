mod parsing;
mod map;
mod map_procedure;
mod heuristics;
mod make_final_grid;

use map_procedure::{core_swap, make_final_grid};
use map::*;
use parsing::*;
use make_final_grid::*;

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
    let final_grid = make_final_grid();

    core_swap(&map, &final_grid, 'd');
    map.print();
    println!("test : {:?}", map);
}
