// File with our heuristics

use std::collections::HashMap;

use crate::map::Map;

//      Goal Map ::
//
//   0:1   1:2   2:3
//   3:8   4:0   5:4
//   6:7   7:6   8:5

//      Current Map :
//
//   0:6   1:8   2:5
//   3:7   4:1   5:0
//   6:2   7:4   8:3

// mahnatan distance : 18

pub fn manhatan_distance(current_map: &Map, goal_map: &HashMap<i32, i32>) -> i32
{
    // calcul index x
    // calcul index y
    let mut distance = 0;
    let mut index = 0;
    for x in current_map.grid.iter()
    {
        if *x != 0 {
            let goal_val = goal_map[&x];
            let x_diff = (goal_val % current_map.width as i32)
                - (index % current_map.width) as i32;
            let y_diff = (goal_val / current_map.width as i32)
                - (index / current_map.width) as i32;
            distance += x_diff.abs() + y_diff.abs();
        }
        index += 1;
    }
    distance
}

pub fn euclidean_distance(current_map: &Map, goal_map: &HashMap<i32, i32>) -> i32 {
    // calcul index x
    // calcul index y
    let mut distance = 0;
    let mut index = 0;
    for x in current_map.grid.iter()
    {
        if *x != 0 {
            let goal_val = goal_map[&x];
            let x_diff = (goal_val % current_map.width as i32)
                - (index % current_map.width) as i32;
            let y_diff = (goal_val / current_map.width as i32)
                - (index / current_map.width) as i32;
            distance += (<f64>::sqrt((<i32>::pow(x_diff.abs(), 2) + <i32>::pow(y_diff.abs(), 2)) as f64)).round() as i32;
        }
        index += 1;
    }
    distance
}

pub fn misplaced_tiles(current_map: &Map, goal_map: &HashMap<i32, i32>) -> i32 {
    // calcul index x
    // calcul index y
    let mut distance = 0;
    let mut index = 0;
    for x in current_map.grid.iter()
    {
        if *x != 0 { if goal_map[&x] != index { distance += 1; }}
        index += 1;
    }
    distance
}

