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
use crate::map_procedure::core_swap;

use std::env;
use std::fs::File;
use std::error::Error;
use std::process::exit;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::io::{stdin, stdout, Write};

type HeuristicType = fn(&Map, &HashMap<i32, i32>) -> i32;

fn  generate_child(current_state: &Map, open: &mut BinaryHeap<Map>,
                  close: &HashMap<String, Map>, goal_map: &HashMap<i32, i32>,
                  heuristic_func: &HeuristicType)
{
    for i in "LURD".chars()
    {
        // check
        let new_map = match i
        {
            'L' =>
            {
                // left
                if current_state.hole % current_state.width > 0
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func))
                }
                else { None }
            },
            'U' =>
            {
                // uo
                if current_state.hole / current_state.width > 0
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func))
                }
                else { None }
            },
            'R' =>
            {
                // right
                if current_state.hole % current_state.width < current_state.width - 1
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func))
                }
                else { None }
            },
            'D' =>
            {
                // down
                if current_state.hole / current_state.width < current_state.height - 1
                {
                    Some(core_swap(current_state, goal_map, i, heuristic_func))
                }
                else { None }
            },
            _ => { panic!("Wrong letters") }
        };
        match new_map
        {
            Some(new_map) =>
            {
                // check if explore
                if close.contains_key(&new_map.get_key())
                {
                    // already explored
                    // go udpate
             //       println!("oulbiez pas de faire le update !!!!");
                } else { open.push(new_map); }
            },
            None => {continue;}
        };
    }
}

fn  expand(initial_state: &Map, goal_map: &HashMap<i32, i32>, heuristic_func: &HeuristicType) -> Map {
    let mut open = BinaryHeap::new();
    let mut close : HashMap<String, Map> = HashMap::new();
    let mut current = initial_state.clone();

    while current.heuristic_value > 0
    {
        generate_child(&current, &mut open, &close, goal_map, heuristic_func);
        //   appel les 4 fonctions swap
        //   pour chaque return :
        //       Regarer si deja explorer, si non push dans opens
        //       sinon verifier que le chemins ne soit pas plus court
        close.insert(current.get_key(), current.clone());
        current = match open.pop() {
            Some(tmp) => tmp,
            None => break
        };
    }
    current
}

fn ask_heuristic() -> HeuristicType
{
    let mut s = String::new();
    println!("Veuillez selectionner un nombre entre 1 et 3 pour choisir votre heuristcs : ");
    println!("- 1 : Manhattan Distance");
    println!("- 2 : Euclidean Distance");
    println!("- 3 : Misplaced Tiles");
    loop
    {
        print!("$> ");
        let _ = stdout().flush();
        s.clear();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if s.len() == 2
        {
            let cur_char = s.chars().next().unwrap();
            if cur_char == '1' || cur_char == '2' || cur_char == '3'
            {
                if cur_char == '1' {
                    return heuristics::manhatan_distance;
                } else if cur_char == '2' {
                    return heuristics::euclidean_distance;
                }
                else { return heuristics::misplaced_tiles; }
                //return cur_char
            }
        }
        println!("Veuillez entrer un chiffre entre 1 et 3");
    }
}

fn  test_solution(initial_map: &Map, final_map: &Map)
{
    let mut copy_state = initial_map.clone();

    for hole_move in final_map.shortest_path.chars()
    for hole_move in test.chars()
    {
        match hole_move
        {
            'U' =>
            {
                copy_state.grid[copy_state.hole] = copy_state.grid[copy_state.hole - copy_state.width];
                copy_state.grid[copy_state.hole - copy_state.width] = 0;
                copy_state.hole -= copy_state.width;
            },
            'L' =>
            {
                copy_state.grid[copy_state.hole] = copy_state.grid[copy_state.hole - 1];
                copy_state.grid[copy_state.hole - 1] = 0;
                copy_state.hole -= 1;
            },
            'D' =>
            {
                copy_state.grid[copy_state.hole] = copy_state.grid[copy_state.hole + copy_state.width];
                copy_state.grid[copy_state.hole + copy_state.width] = 0;
                copy_state.hole += copy_state.width;
            },
            'R' =>
            {
                copy_state.grid[copy_state.hole] = copy_state.grid[copy_state.hole + 1];
                copy_state.grid[copy_state.hole + 1] = 0;
                copy_state.hole += 1;
            }
            _ => panic!("stop, something wrong.")
        }
        copy_state.print();
        println!();
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("");
    }
    println!("End in {} moves: {}", final_map.shortest_path.len(), final_map.shortest_path);
}

fn  main()
{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { println!("usage: ./npuzzle [puzzle]"); exit(0); }
    let mut map: Map = Map::new();
    let file = match File::open(&args[1])
    {
        Ok(file) => file,
        Err(e) => panic!("error: {}", e.description())
    };

    let heuristic_func = ask_heuristic();
    parse(&mut map, file);

    println!("Begin State:");
    map.print();
    println!();

    // algo
    let final_grid = make_final_grid(map.width as i32, map.height as i32);
    // expand tous les enfants ?
    let mut final_state = Map::new();
    if is_doable(&map, &final_grid) == 0
    {
        final_state = expand(&map, &final_grid, &heuristic_func);
    }
    else { println!("undoable"); }
    println!();
    test_solution(&map, &final_state);
}
