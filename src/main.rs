use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec::Vec;

#[allow(non_snake_case)]
trait Funct
{
    fn print(&self);
    fn getFinalCoord(&self, value: i32) -> i32;
    fn swap(&self, action: char) -> State;
}

#[allow(non_snake_case)]
struct State
{
    size: usize,
    grid: Vec<Vec<i32>>,
    hole: (i32, i32),
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

    fn getFinalCoord(&self, value: i32) -> i32
    {
        for i in 0..usize::pow(self.size, 2)
        {
            if self.finalGrid[i / self.size][i % self.size] == value
            {
                return i as i32;
            }
        }
        -1
    }

    fn swap(&self, action: char) -> State
    {
        let mut newState = self.clone();
        let mut x = 0;
        let mut y = 0;

        if (action == 'E' && self.hole.0 < self.size as i32 - 1) || (action == 'W' && self.hole.0 > 0)
        {
            x = if action == 'E' { 1 } else { -1 };
        }
        if (action == 'S' && self.hole.1 < self.size as i32 - 1) || (action == 'N' && self.hole.1 > 0)
        {
            y = if action == 'S' { 1 } else { -1 };
        }
        /*
        newState.grid[self.hole.1 as usize][self.hole.0 as usize] = newState.grid[(self.hole.1 + y) as usize][(self.hole.0 + x) as usize];
        newState.grid[(self.hole.1 + y) as usize][(self.hole.0 + x) as usize] = 0;
        newState.hole.0 += x;
        newState.hole.1 += y;
        */

        State { size: 0, grid: vec![vec![0i32; 4]; 4], hole: (-1, -1), finalGrid: vec![vec![0i32; 4]; 4] }
    }
}

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
    grid
}

#[allow(non_snake_case)]
fn      createFirstState(file: File) -> State
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
    State { size: size, grid: grid, hole: hole, finalGrid: makeFinalGrid(size) }
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
    firstState.swap('E');
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
