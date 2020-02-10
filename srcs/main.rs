mod map;
mod parsing;
mod is_doable;
mod heuristics;
mod map_procedure;
mod make_final_grid;

use map::*;
use parsing::*;
use is_doable::*;
use make_final_grid::*;
use map_procedure::core_swap;

use std::env;
use std::fs::File;
use std::error::Error;
use std::process::exit;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::io::{stdin, stdout, Write};

type heuristic_type = fn(&Map, &HashMap<i32, i32>) -> i32;

fn  generate_child(current_state: &Map, open: &mut BinaryHeap<Map>,
                  close: &HashMap<String, Map>, goal_map: &HashMap<i32, i32>,
                  heuristic_func: &heuristic_type)
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

fn  expand(initial_state: &Map, goal_map: &HashMap<i32, i32>, heuristic_func: &heuristic_type) -> Map {
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

fn ask_heuristic() -> heuristic_type
{
    let mut s = String::new();
    println!("Veuillez selectionner un nombre entre 1 et 3 pour choisir votre heuristcs : ");
    println!("- 1 : Manhattan Distance");
    println!("- 2 : Euclidean Distance");
    println!("- 3 : Misplaced Tiles");
    loop
    {
        print!("$> ");
        stdout().flush();
        s.clear();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if (s.len() == 2)
        {
            let cur_char = s.chars().next().unwrap();
            if (cur_char == '1' || cur_char == '2' || cur_char == '3')
            {
                if (cur_char == '1') {
                    return heuristics::manhatan_distance;
                } else if (cur_char == '2') {
                    return heuristics::euclidean_distance;
                }
                else { return heuristics::misplaced_tiles; }
                //return cur_char
            }
        }
        println!("Veuillez entrer un chiffre entre 1 et 3");
    }
}

fn  main()
{
    let args: Vec<String> = env::args().collect();
    let mut map: Map = Map::new();

    if args.len() != 2 { println!("usage: ./npuzzle [puzzle]"); exit(0); }

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
    if is_doable(&map, &final_grid) == 0
    {
        let end_state = expand(&map, &final_grid, &heuristic_func);
        println!("End State: ");
        end_state.print();
        println!("End in {} moves: {}", end_state.shortest_path.len(), end_state.shortest_path);
    }
    else { println!("undoable"); }
}
