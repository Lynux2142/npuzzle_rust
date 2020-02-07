mod parsing;
mod map;
mod map_procedure;
mod heuristics;
mod make_final_grid;

use std::collections::BinaryHeap;
use std::collections::HashMap;

use map_procedure::{core_swap, make_final_grid};
use map::*;
use parsing::*;
use make_final_grid::*;

use map::Map;
use std::env;
use std::fs::File;
use std::error::Error;

fn generate_child(current_state: & Map, open: &mut BinaryHeap<Map>,
                  close: &HashMap<String, Map>, goal_map: &HashMap<i32, i32>)
{
    for i in "lurd".chars() {
        // check
        let new_map: Option<Map> = match i {
            'l' => {
                // left
                if current_state.hole % current_state.width > 0 {
                    Some(core_swap(current_state, goal_map, i))
                } else {
                    None
                }
            },
            'u' => {
                // uo
                if current_state.hole / current_state.width > 0 {
                    Some(core_swap(current_state, goal_map, i))
                } else {
                    None
                }
            },
            'r' => {
                // right
                if current_state.hole % current_state.width < current_state.width - 1 {
                    Some(core_swap(current_state, goal_map, i))
                } else {
                    None
                }
            },
            'd' => {
                // down
                if current_state.hole / current_state.width < current_state.height - 1 {
                    Some(core_swap(current_state, goal_map, i))
                } else {
                    None
                }
            },
            _ => { panic!("Wrong letters") }
        };
        match new_map {
            Some(new_map) =>
            {
                // check if explore
                if close.contains_key(&new_map.get_key()) {
                    // already explored
                    // go udpate
             //       println!("oulbiez pas de faire le update !!!!");
                } else {
                    open.push(new_map);
                    // ajouter a open
                }
            },
            None => {continue;}
        };
    }
}

fn expand(initial_state: & Map, goal_map: &HashMap<i32, i32>) {
    let mut open = BinaryHeap::new();
    let mut close : HashMap<String, Map> = HashMap::new();
    let mut current = initial_state.clone();

    let mut i = 0;
    while current.heuristic_value > 0 {

        generate_child(&current, &mut open, &close, goal_map);

        //   appel les 4 fonctions swap
        //   pour chaque return :
        //       Regarer si deja explorer, si non push dans opens
        //       sinon verifier que le chemins ne soit pas plus court
        close.insert(current.get_key(), current.clone());
/*        if (i == 2) {
            println!("\nopen : {:?}", open);

            println!("close : {:?}\n", close);
            break ;

        }*/
        current = match open.pop() {
            Some(tmp) => tmp,
            None => break
        };

        i += 1;
    }
    println!("heuristic : {}", heuristics::manhatan_distance(&current, goal_map));
    println!("goal Map : {:?}", goal_map);
    println!("voici notre solution : ");
    current.print();
    println!("shortest path : {}", current.shortest_path);
}

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


    // algo
    let final_grid = make_final_grid();
    // expand tous les enfants ?
    expand(&map, &final_grid);

}
