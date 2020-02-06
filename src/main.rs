mod map;
use map::Map;

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
fn core_swap(current_map: & Map, direction: char) -> Map {
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
    // on va effectuer le swap ici
    println!("voici la new_map : {:?}", new_map);
    Map::new()
}

//          5   6   7
//          1   0   3
//          4   2   8


fn main() {
    let mut test = Map::new();

    test.width = 3;
    test.height = 3;

    test.grid = vec![5,6,7,1,0,3,4,2,8];
    test.hole = 4;

    println!("test : {:?}", test);
    core_swap(&test, 'r');
}
