use std::collections::HashMap;

use crate::map::Map;

type HeuristicType = fn(&Map, &HashMap<i32, i32>) -> i32;

pub fn swap(current_map: &Map, direction: char) -> Map
{
    let mut new_map = current_map.clone();
    let gap: i32;

    match direction
    {
        'L' | 'R' => { gap = if direction == 'R' { 1 } else { -1 }; },
        _ => { gap = if direction == 'D' { new_map.width } else { -new_map.width } }
    };
    new_map.grid[new_map.hole as usize] = new_map.grid[(new_map.hole + gap) as usize];
    new_map.grid[(new_map.hole + gap) as usize] = 0;
    new_map.hole += gap;
    new_map.shortest_path.push(direction);
    new_map
}

pub fn core_swap(current_map: &Map, goal_map: &HashMap<i32, i32>, direction: char,
                 heuristic_func: &HeuristicType, algo_char: char) -> Map
{
    let mut new_map = swap(current_map, direction);
    new_map.heuristic_value = heuristic_func(&new_map, goal_map);
    if algo_char == 'a'
    {
        new_map.cost = new_map.shortest_path.len() as i32 + new_map.heuristic_value;
    }
    else if algo_char == 'b' { new_map.cost = new_map.heuristic_value; }
    else { new_map.cost = new_map.shortest_path.len() as i32; }
    new_map
}
