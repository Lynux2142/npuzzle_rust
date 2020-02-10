use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::map::*;
use std::process::exit;

fn      get_size(width: &mut usize, height: &mut usize, line: String)
{
    let values = line.split(" ").collect::<Vec<&str>>();

    match values.len()
    {
        1 =>
        {
            let size = match values[0].parse::<usize>()
            {
                Ok(size) => { size },
                Err(e) => {
                    println!("error: {}", e);
                    exit(1);
                }
            };
            *width = size;
            *height = size;
        },
        2 =>
        {
            let size = match values[0].parse::<usize>()
            {
                Ok(size) => { size },
                Err(e) => {
                    println!("error: {}", e);
                    exit(1);
                }
            };
            *width = size;
            let size = match values[1].parse::<usize>()
            {
                Ok(size) => { size },
                Err(e) => {
                    println!("error: {}", e);
                    exit(1);
                }
            };
            *height = size;
        }
        _ => {
            println!("error");
            exit(4);
        }
    }
}

fn      set_values(grid: &mut Vec<i32>, width: usize, y: usize, values: std::str::SplitWhitespace) -> i32
{
    let mut hole = -1i32;
    let mut x = 0usize;

    for value in values
    {
        let tmp = match value.parse::<i32>()
        {
            Ok(tmp) => tmp,
            Err(e) => {
                println!("error: {}", e);
                exit(1);
            }
        };
        grid[y * width + x] = tmp;
        if tmp == 0 { hole = (y * width + x) as i32; }
        x += 1;
    }
    hole
}

pub fn  parse(map: & mut Map, file: File)
{
    let reader = BufReader::new(file);
    let mut y = 0usize;

    for line in reader.lines()
    {
        let line = match line
        {
            Ok(line) => line,
            Err(e) => {
                println!("error: {}", e);
                exit(1);
            }
        };

        match line.chars().next()
        {
            Some(is_com) =>
            {
                if is_com != '#'
                {
                    if map.width == 0usize
                    {
                        get_size(&mut map.width, &mut map.height, line);
                        map.grid.resize(map.width * map.height, 0);
                    }
                    else
                    {
                        let tmp = set_values(&mut map.grid, map.width, y, line.split_whitespace());
                        if tmp != -1i32 { map.hole = tmp as usize; }
                        y += 1;
                    }
                }
            },
            None => break,
        }
    }
    map.size = map.width * map.height;
}
