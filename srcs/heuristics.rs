use std::collections::HashMap;

use crate::map::Map;

pub fn manhatan_distance(current_map: &Map, goal_map: &HashMap<i32, i32>) -> i32
{
    let mut distance = 0;
    let mut index = 0;
    for x in current_map.grid.iter()
    {
        if *x != 0
        {
            let goal_val = goal_map[&x];
            let x_diff = (goal_val % current_map.width)
                - (index % current_map.width);
            let y_diff = (goal_val / current_map.width)
                - (index / current_map.width);
            distance += x_diff.abs() + y_diff.abs();
        }
        index += 1;
    }
    distance
}

pub fn euclidean_distance(current_map: &Map, goal_map: &HashMap<i32, i32>) -> i32 {
    let mut distance = 0;
    let mut index = 0;
    for x in current_map.grid.iter()
    {
        if *x != 0
        {
            let goal_val = goal_map[&x];
            let x_diff = (goal_val % current_map.width)
                - (index % current_map.width);
            let y_diff = (goal_val / current_map.width)
                - (index / current_map.width);
            distance += <f32>::sqrt((x_diff.abs().pow(2) + y_diff.abs().pow(2)) as f32).round() as i32;
        }
        index += 1;
    }
    distance
}

pub fn misplaced_tiles(current_map: &Map, goal_map: &HashMap<i32, i32>) -> i32 {
    let mut distance = 0;
    let mut index = 0;
    for x in current_map.grid.iter()
    {
        if *x != 0 && goal_map[&x] != index { distance += 1; }
        index += 1;
    }
    distance
}
