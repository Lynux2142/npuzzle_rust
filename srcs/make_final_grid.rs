use std::collections::HashMap;

pub fn make_final_grid(width: i32, height: i32) -> HashMap<i32, i32>
{
    let mut final_grid = HashMap::new();
    let size = width * height;
    let mut x_min_max = ( 0i32, width - 1 );
    let mut y_min_max = ( 0i32, height - 1 );
    let mut i = 1i32;

    while i <= size
    {
        for x in x_min_max.0..=x_min_max.1
        {
            if i > size { break; }
            final_grid.insert(if i == size { 0 } else { i }, y_min_max.0 * width + x);
            i += 1;
        }
        y_min_max.0 += 1;
        for y in y_min_max.0..=y_min_max.1
        {
            if i > size { break; }
            final_grid.insert(if i == size { 0 } else { i }, y * width + x_min_max.1);
            i += 1;
        }
        x_min_max.1 -= 1;
        for x in (x_min_max.0..=x_min_max.1).rev()
        {
            if i > size { break; }
            final_grid.insert(if i == size { 0 } else { i }, y_min_max.1 * width + x);
            i += 1;
        }
        y_min_max.1 -= 1;
        for y in (y_min_max.0..=y_min_max.1).rev()
        {
            if i > size { break; }
            final_grid.insert(if i == size { 0 } else { i }, y * width + x_min_max.0);
            i += 1;
        }
        x_min_max.0 += 1;
    }
    final_grid
}
