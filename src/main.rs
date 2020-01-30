use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec::Vec;

trait Funct
{
    fn print(&self);
}

struct State
{
    size: usize,
    grid: Vec<Vec<i32>>
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
    return State { size: size, grid: grid };
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
    }
    Ok(())
}
