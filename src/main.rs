mod state;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec::Vec;
use state::State;

#[allow(non_snake_case)]
fn      createFirstState(firstState: &mut State, file: File)
{
    let reader = BufReader::new(file);
    let mut size: usize = 0;
    let mut grid = vec![vec![0i32; 1]; 1];
    let mut y = 0;
    let mut hole = (-1, -1);

    for line in reader.lines()
    {
        let line = line.unwrap();

        if line.chars().next().unwrap() != '#'
        {
            if size == 0 { size = line.parse::<usize>().unwrap(); grid = vec![vec![0i32; size]; size]; }
            else
            {
                let i = line.split_whitespace();
                let mut x = 0;
                for value in i
                {
                    grid[y][x] = value.parse::<i32>().unwrap();
                    if grid[y][x] == 0
                    {
                        hole.0 = x as i32;
                        hole.1 = y as i32;
                    }
                    x += 1;
                }
                y += 1;
            }
        }
    }
    firstState.init(size, grid);
}

#[allow(non_snake_case)]
fn      isItDoable(firstState: &State) -> i32
{
    let n;
    let mut j = -1;
    let mut np;
    let mut copie = vec![0i32; i32::pow(firstState.size as i32, 2) as usize];

    for i in 0..(i32::pow(firstState.size as i32, 2))
    {
        copie[i as usize] = firstState.grid[(i / firstState.size as i32) as usize][(i % firstState.size as i32) as usize];
        if copie[i as usize] == 0
        {
            j = i;
        }
    }
    n = firstState.getFinalCoord(0);
    np = i32::abs(n % firstState.size as i32 - j % firstState.size as i32) + i32::abs(n / firstState.size as i32 - j / firstState.size as i32);
    for n in (1..i32::pow(firstState.size as i32, 2)).rev()
    {
        let test = firstState.getFinalCoord(copie[j as usize]);
        if test != j
        {
            let tmp = copie[j as usize];

            copie[j as usize] = copie[firstState.getFinalCoord(tmp) as usize];
            copie[firstState.getFinalCoord(tmp) as usize] = tmp;
            np += 1;
        }
        j = 0;
        while copie[j as usize] != n
        {
            j += 1;
        }
    }
    1 & np
}

#[allow(non_snake_case)]
fn      resolve(firstState: &State)
{
    let mut open: Vec<&State> = Vec::new();

    open.push(firstState);
    while open[0].heuristic > 0
    {
        open[0].swap('N').print();
        break;
    }
    open.clear();
}

#[allow(non_snake_case)]
fn      main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    let file;
    let mut firstState = State::new();

    if args.len() != 2 { println!("usage: cargo run [puzzle]"); }
    else
    {
        file = File::open(&args[1])?;

        createFirstState(&mut firstState, file);
        println!("First State:");
        firstState.print();
        if isItDoable(&firstState) == 0
        {
            resolve(&firstState);
        }
        else
        {
            println!("Not Doable");
        }
    }
    Ok(())
}
