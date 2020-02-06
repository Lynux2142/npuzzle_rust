use std::io::{BufRead, BufReader};
use std::fs::File;
use map::*;

fn          get_size(width: &mut usize, height: &mut usize, line: String)
{
    let size;

    size = match line.parse::<usize>()
    {
        Ok(size) =>
        {
            size
        },
        Err(e) => panic!("error: {}", e)
    };
    *width = size;
    *height = size;
}

fn          set_values(grid: &mut Vec<i32>, width: usize, y: usize, values: std::str::SplitWhitespace)
{
    let mut x = 0usize;

    for value in values
    {
        let tmp = match value.parse::<i32>()
        {
            Ok(tmp) => tmp,
            Err(e) => panic!("error: {}", e)
        };
        grid[y * width + x] = tmp;
        x += 1;
    }
}

pub fn      parse(map: & mut Map, file: File)
{
    let reader = BufReader::new(file);
    let mut width = 0usize;
    let mut height = 0usize;
    let mut y = 0usize;
    let mut grid: Vec<i32> = Vec::new();

    for line in reader.lines()
    {
        let line = match line
        {
            Ok(line) => line,
            Err(e) => panic!("error: {}", e)
        };

        match line.chars().next()
        {
            Some(is_com) =>
            {
                if is_com != '#'
                {
                    if width == 0usize
                    {
                        get_size(&mut width, &mut height, line);
                        grid.resize(width * height, 0);
                        println!("width: {} - height: {}", width, height);
                    }
                    else
                    {
                        set_values(&mut grid, width, y, line.split_whitespace());
                        y += 1;
                    }
                }
            },
            None => break,
        }
    }
    map.size = width * height;
    map.width = width;
    map.height = height;
    map.grid = grid.clone()
}
