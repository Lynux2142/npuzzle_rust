use std::collections::HashMap;
use crate::map::*;

pub fn  is_doable(map: &Map, final_grid: &HashMap<i32, i32>) -> i32
{
    let mut j: i32 = -1;
    let n = final_grid[&0];
    let mut grid = map.grid.clone();
    for i in 0..map.size { if grid[i as usize] == 0 { j = i; } }
    let mut np = <i32>::abs(n % map.width - j % map.width) +
        <i32>::abs(n / map.width - j / map.width);
    for n in (1..(map.size)).rev()
    {
        let test = final_grid[&grid[j as usize]];
        if test != j
        {
            let tmp = grid[j as usize];

            grid[j as usize] = grid[final_grid[&tmp] as usize];
            grid[final_grid[&tmp] as usize] = tmp;
            np += 1;
        }
        j = 0;
        while grid[j as usize] != n { j += 1; }
    }
    1 & np
}
