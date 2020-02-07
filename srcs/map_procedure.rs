use std::collections::HashMap;

use map::Map;

// trouver un meilleur moyen de lui filer les heuristics
// pour etre un peu plus modulaire
use heuristics::manhatan_distance;

#[allow(dead_code)]
pub fn      make_final_grid() -> HashMap<i32, i32>
{
    let mut final_grid = HashMap::new();
    final_grid.insert(0, 4);
    final_grid.insert(1, 0);
    final_grid.insert(2, 1);
    final_grid.insert(3, 2);
    final_grid.insert(4, 5);
    final_grid.insert(5, 8);
    final_grid.insert(6, 7);
    final_grid.insert(7, 6);
    final_grid.insert(8, 3);
    final_grid
//    vec![1,2,3,8,0,4,7,6,5]
}

#[allow(dead_code)]
fn get_index(grid: &Vec<i32>, to_search: i32) -> Option<usize> {
    for i in 0..grid.len() {
        if grid[i] == to_search {
            return Some(i)
        }
    }
    None
}

fn swap(to_swap_map: &mut Vec<i32>, hole_index: &i32, index_diff: &i32)  {
    let tmp = to_swap_map[(hole_index + index_diff) as usize];
    to_swap_map[*hole_index as usize] = tmp;
    to_swap_map[(hole_index + index_diff) as usize] = 0; // hole
}

// take a ref to a map struct and a direction
// generate another map state from this
pub fn core_swap(current_map: & Map, goal_map: & HashMap<i32, i32>, direction: char) -> Map {
    let mut new_map = Map::new();
    // init new_map, a voir pour implementer le Trait clone
    new_map.size = current_map.size;
    new_map.width = current_map.width;
    new_map.height = current_map.height;
    new_map.hole = current_map.hole;
    new_map.heuristic_value = current_map.heuristic_value;
    new_map.cost = current_map.cost;
    new_map.grid = current_map.grid.clone();

    let mut new_grid = &mut new_map.grid;
    match direction
    {
        'l' | 'L' => {
            // left
            swap(&mut new_grid, &current_map.hole, &-1)
        },
        'u' | 'U' => {
            // up
            swap(&mut new_grid, &current_map.hole, &-(current_map.width as i32))
        },
        'r' | 'R' => {
            // right
            swap(&mut new_grid, &current_map.hole, &1)
        },
        'd' | 'D' => {
            // down
            swap(&mut new_grid, &current_map.hole, &(current_map.width as i32))
        },
        _ => {
            panic!("Wrong letters");
            // alors je me permet un panic car on est vraiment
            // pas sense envoye une mauvaise lettre ...
        }
    };
    new_map.heuristic_value = manhatan_distance(&new_map, goal_map);
    new_map
}
