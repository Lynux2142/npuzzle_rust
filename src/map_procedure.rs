use map::Map;

#[allow(dead_code)]
fn      make_final_grid(size: usize) -> Vec<Vec<i32>>
{
    let mut grid = vec![vec![0i32; size]; size];
    let end = usize::pow(size, 2);
    let mut x_min_max = (0, size as i32 - 1);
    let mut y_min_max = (0, size as i32 - 1);
    let mut i = 0;
    let mut x;
    let mut y;

    while i < end
    {
        x = x_min_max.0;
        while x <= x_min_max.1 && i < end
        {
            i += 1;
            grid[y_min_max.0 as usize][x as usize] = if i < end { i as i32 } else { 0 };
            x += 1;
        }
        y_min_max.0 += 1;

        y = y_min_max.0;
        while y <= y_min_max.1 && i < end
        {
            i += 1;
            grid[y as usize][x_min_max.1 as usize] = if i < end { i as i32 } else { 0 };
            y += 1;
        }
        x_min_max.1 -= 1;

        x = x_min_max.1;
        while x >= x_min_max.0 && i < end
        {
            i += 1;
            grid[y_min_max.1 as usize][x as usize] = if i == end { 0 } else { i as i32 };
            x -= 1;
        }
        y_min_max.1 -= 1;

        y = y_min_max.1;
        while y >= y_min_max.0 && i < end
        {
            i += 1;
            grid[y as usize][x_min_max.0 as usize] = if i == end { 0 } else { i as i32 };
            y -= 1;
        }
        x_min_max.0 += 1;
    }
    grid
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
pub fn core_swap(current_map: & Map, direction: char) -> Map {
    let mut new_map = Map::new();
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

    println!("voici la new_map : {:?}", new_map);
    Map::new()
}
/*
fn main() {
    let mut test = Map::new();

    test.width = 3;
    test.height = 3;

    test.grid = vec![5,6,7,1,0,3,4,2,8];
    test.hole = 4;

    println!("test : {:?}", test);
    core_swap(&test, 'r');
}*/
