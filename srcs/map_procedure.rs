use std::collections::HashMap;

use map::Map;

enum Sign
{
    POSITIF,
    NEGATIF
}

// trouver un meilleur moyen de lui filer les heuristics
// pour etre un peu plus modulaire
use heuristics::manhatan_distance;

/*
#[allow(dead_code)]
fn get_index(grid: &Vec<i32>, to_search: i32) -> Option<usize> {
    for i in 0..grid.len() {
        if grid[i] == to_search {
            return Some(i)
        }
    }
    None
}
*/

fn swap(to_swap_map: &mut Vec<i32>, hole_index: &usize, index_diff: &usize, sign: Sign)
{
    match sign
    {
        Sign::POSITIF =>
        {
            let tmp = to_swap_map[(hole_index + index_diff)];
            to_swap_map[*hole_index] = tmp;
            to_swap_map[(hole_index + index_diff)] = 0; // hole

        },
        Sign::NEGATIF =>
        {
            let tmp = to_swap_map[(hole_index - index_diff)];
            to_swap_map[*hole_index] = tmp;
            to_swap_map[(hole_index - index_diff)] = 0; // hole
        }
    }
}

// take a ref to a map struct and a direction
// generate another map state from this
pub fn core_swap(current_map: &Map, goal_map: &HashMap<i32, i32>, direction: char) -> Map
{
    let mut new_map = current_map.clone();
    // init new_map, a voir pour implementer le Trait clone

    // shortest path
    new_map.shortest_path.push(direction);

    let mut new_grid = &mut new_map.grid;
    match direction
    {
        'L' =>
        {
            // left
            swap(&mut new_grid, &current_map.hole, &1, Sign::NEGATIF);
            new_map.hole -= 1;
        },
        'U' =>
        {
            // up
            swap(&mut new_grid, &current_map.hole, &current_map.width, Sign::NEGATIF);
            new_map.hole -= current_map.width;
        },
        'R' =>
        {
            // right
            swap(&mut new_grid, &current_map.hole, &1, Sign::POSITIF);
            new_map.hole += 1;
        },
        'D' =>
        {
            // down
            swap(&mut new_grid, &current_map.hole, &current_map.width, Sign::POSITIF);
            new_map.hole += current_map.width;
        },
            // alors je me permet un panic car on est vraiment
            // pas sense envoye une mauvaise lettre ...
        _ => { panic!("Wrong letters"); }
    };
    new_map.heuristic_value = manhatan_distance(&new_map, goal_map);
    new_map.cost = new_map.shortest_path.len() as i32 + new_map.heuristic_value;
    new_map
}
