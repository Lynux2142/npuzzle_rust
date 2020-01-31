use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec::Vec;

trait Funct
{
    fn print(&self);
}

#[allow(non_snake_case)]
struct State
{
    size: usize,
    grid: Vec<Vec<i32>>,
    finalGrid: Vec<Vec<i32>>
}

#[allow(non_snake_case)]
impl Funct for State
{
    fn print(&self)
    {
        let mut holeSize = 0;
        let mut valueSize;
        let mut tmp: i32 = (self.size * self.size) as i32;

        while tmp >= 10 { holeSize += 1; tmp /= 10; }

        for line in self.grid.iter()
        {
            for elem in line.iter()
            {
                valueSize = 0;
                if *elem != 0
                {
                    tmp = *elem;
                    while tmp >= 10 { valueSize += 1; tmp /= 10; }
                }
                for _ in valueSize..holeSize { print!(" "); }
                print!("{} ", elem);
            }
            println!();
        }
    }
}

#[allow(non_snake_case)]
fn      createFirstState(file: File) -> State
{
    let reader = BufReader::new(file);
    let mut size: usize = 0;
    let mut grid = vec![vec![0i32; 1]; 1];
    let mut y = 0;

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
                    x += 1;
                }
                y += 1;
            }
        }
    }
    return State { size: size, grid: grid, finalGrid: makeFinalGrid(size) };
}

/*
#[allow(non_snake_case)]
fn      isItDoable(firstState: State) -> i32
{
    let mut n = 9;
    let mut j = 0;
    let mut np = 0;
    let mut copie = vec![0i32; i32::pow(firstState.size as i32, 2) as usize];

    for i in 0..(i32::pow(firstState.size as i32, 2))
    {
        copie[i as usize] = firstState.grid[(i / firstState.size as i32) as usize][(i % firstState.size as i32) as usize];
    }
    np = i32::abs(n % firstState.size as i32 - j % firstState.size as i32) + i32::abs(n / firstState.size as i32 - j % firstState.size as i32);
    return 1 & np;
}
*/

#[allow(non_snake_case)]
fn      makeFinalGrid(size: usize) -> Vec<Vec<i32>>
{
    let mut grid = vec![vec![0i32; size]; size];
    let end = usize::pow(size, 2);
    let mut xMinMax = (0, size as i32 - 1);
    let mut yMinMax = (0, size as i32 - 1);
    let mut i = 0;
    let mut x;
    let mut y;

    while i < end
    {
        x = xMinMax.0;
        while x <= xMinMax.1 && i < end
        {
            i += 1;
            grid[yMinMax.0 as usize][x as usize] = if i < end { i as i32 } else { 0 };
            x += 1;
        }
        yMinMax.0 += 1;

        y = yMinMax.0;
        while y <= yMinMax.1 && i < end
        {
            i += 1;
            grid[y as usize][xMinMax.1 as usize] = if i < end { i as i32 } else { 0 };
            y += 1;
        }
        xMinMax.1 -= 1;

        x = xMinMax.1;
        while x >= xMinMax.0 && i < end
        {
            i += 1;
            grid[yMinMax.1 as usize][x as usize] = if i == end { 0 } else { i as i32 };
            x -= 1;
        }
        yMinMax.1 -= 1;

        y = yMinMax.1;
        while y >= yMinMax.0 && i < end
        {
            i += 1;
            grid[y as usize][xMinMax.0 as usize] = if i == end { 0 } else { i as i32 };
            y -= 1;
        }
        xMinMax.0 += 1;
    }
    return grid;
}

#[allow(non_snake_case)]
fn      main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    let file;
    let firstState;

    if args.len() != 2 { println!("usage: cargo run [puzzle]"); }
    else
    {
        file = File::open(&args[1])?;

        firstState = createFirstState(file);
        firstState.print();
        println!();
        for line in firstState.finalGrid
        {
            println!("{:?}", line);
        }
    }
    Ok(())
}
