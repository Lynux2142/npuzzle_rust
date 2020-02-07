use std::collections::HashMap;
use map::*;

pub fn  is_doable(map: &Map, final_grid: &HashMap<i32, i32>) -> i32
{
    let mut j = -1i32;
    let n = final_grid[&0i32];
    let mut grid = map.grid.clone();
    for i in 0..map.size { if grid[i] == 0 { j = i as i32; } }
    let mut np = <i32>::abs(n % map.width as i32 - j % map.width as i32) +
        <i32>::abs(n / map.width as i32 - j / map.width as i32);
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
        while grid[j as usize] != n as i32 { j += 1; }
    }
    1 & np
}
