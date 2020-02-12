mod map;
mod parsing;
mod is_doable;
mod heuristics;
mod map_procedure;
mod make_final_grid;

use crate::map::*;
use crate::parsing::*;
use crate::is_doable::*;
use crate::make_final_grid::*;
use crate::map_procedure::*;

use std::env;
use std::fs::File;
use std::error::Error;
use std::process::exit;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::io::{stdin, stdout, Write};

type HeuristicType = fn(&Map, &HashMap<i32, i32>) -> i32;

fn  generate_child(current_state: &Map, open: &mut BinaryHeap<Map>,
                   close: &mut HashMap<String, Map>, goal_map: &HashMap<i32, i32>,
                   heuristic_func: &HeuristicType, algo_char: char)
{
    for i in "LURD".chars()
    {
        let new_map = match i
        {
            'L' =>
            {
                if current_state.hole % current_state.width > 0
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func, algo_char))
                }
                else { None }
            },
            'U' =>
            {
                if current_state.hole / current_state.width > 0
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func, algo_char))
                }
                else { None }
            },
            'R' =>
            {
                if current_state.hole % current_state.width < current_state.width - 1
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func, algo_char))
                }
                else { None }
            },
            'D' =>
            {
                if current_state.hole / current_state.width < current_state.height - 1
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func, algo_char))
                }
                else { None }
            },
            _ => panic!("Wrong letters")
        };
        match new_map
        {
            Some(new_map) =>
            {
                if close.contains_key(&new_map.get_key())
                {
                    if new_map.cost < close[&new_map.get_key()].cost
                    {
                        close.remove(&new_map.get_key());
                        open.push(new_map);
                    }
                }
                else { open.push(new_map); }
            },
            None => { continue; }
        };
    }
}

fn  expand(initial_state: &Map, goal_map: &HashMap<i32, i32>, heuristic_func: &HeuristicType, algo_char: char) -> (u32, u32, Map)
{
    let mut open = BinaryHeap::new();
    let mut close : HashMap<String, Map> = HashMap::new();
    let mut current = initial_state.clone();
    let mut time_complexity: u32 = 0;
    let mut size_complexity: u32 = 0;

    while current.heuristic_value > 0
    {
        time_complexity += 1;
        print!("\rtime complexity : {}", time_complexity);
        generate_child(&current, &mut open, &mut close, goal_map, heuristic_func, algo_char);
        if open.len() as u32 > size_complexity { size_complexity = open.len() as u32; }
        close.insert(current.get_key(), current.clone());
        current = match open.pop() {
            Some(tmp) => { tmp },
            None => break
        };
    }
    println!();
    (size_complexity, time_complexity, current)
}

fn ask_algorithm() -> char
{
    let mut s = String::new();
    println!("Please choose a caracter between a and c in order to choose your algorithm : ");
    println!("- a : A * ( Default alogrithm )");
    println!("- b : greedy cost");
    println!("- c : uniform cost");
    loop
    {
        print!("$> ");
        let _ = stdout().flush();
        s.clear();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let cur_char = s.chars().next().unwrap();
        if s.len() == 2
        {
            if cur_char == 'a' || cur_char == 'b' || cur_char == 'c' { return cur_char; }
        }
        else if cur_char == '\n' { return 'a'; }
        println!("choose a caracter between a and c");
    }
}

fn ask_heuristic() -> HeuristicType
{
    let mut s = String::new();
    println!("Please choose a number between 1 and 3 in order to choose your heuristic : ");
    println!("- 1 : Manhattan Distance ( Default Heuristic )");
    println!("- 2 : Euclidean Distance");
    println!("- 3 : Misplaced Tiles");
    loop
    {
        print!("$> ");
        let _ = stdout().flush();
        s.clear();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let cur_char = s.chars().next().unwrap();
        if s.len() == 2
        {
            if cur_char == '1' || cur_char == '2' || cur_char == '3'
            {
                if cur_char == '1' { return heuristics::manhatan_distance; }
                else if cur_char == '2' { return heuristics::euclidean_distance; }
                else { return heuristics::misplaced_tiles; }
            }
        }
        else if cur_char == '\n' { return heuristics::manhatan_distance; }
        println!("Enter a valid number between 1 and 3");
    }

}

fn  test_solution(initial_map: &Map, final_map: &Map, is_manual: bool)
{
    let mut copy_state = initial_map.clone();

    for direction in final_map.shortest_path.chars()
    {
        copy_state = swap(&copy_state, direction);
        copy_state.print();
        println!();
        if is_manual
        {
            let mut s = String::new();
            let _ = stdout().flush();
            stdin().read_line(&mut s).expect("");
        }
    }
    println!("End in {} moves: {}", final_map.shortest_path.len(), final_map.shortest_path);
}

fn  main()
{
    let args: Vec<String> = env::args().collect();
    let mut is_manual = false;
    if args.len() == 3 {
        if !(args[2] == "--manual")
        {
            println!("usage: ./npuzzle puzzle [--manual]\n\t--manual : print states step by step");
        }
        else { is_manual = true; }
    }
    if args.len() != 2 && args.len() != 3
    {
        println!("usage: ./npuzzle puzzle [--manual]\n\t--manual : print states step by step");
        exit(0)
    }
    let mut map: Map = Map::new();
    let file = match File::open(&args[1])
    {
        Ok(file) => file,
        Err(e) =>
        {
            println!("error : {}", e.description());
            exit(1);
        }
    };
    parse(&mut map, file);
    println!("Begin State:");
    map.print();
    println!();
    let final_grid = make_final_grid(map.width, map.height);

    if is_doable(&map, &final_grid) == 0
    {
        let algo_char = ask_algorithm();
        let mut heuristic_func: HeuristicType = heuristics::misplaced_tiles;
        if algo_char != 'c' { heuristic_func = ask_heuristic(); }
        println!();

        let final_state = expand(&map, &final_grid, &heuristic_func, algo_char);
        println!();
        test_solution(&map, &final_state.2, is_manual);
        println!("time_complexity : {}", final_state.1);
        println!("size_complexity : {}", final_state.0);
    }
    else { println!("undoable"); }
}
