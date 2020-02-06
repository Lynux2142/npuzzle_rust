#[allow(non_snake_case)]
pub struct State
{
    pub size: usize,
    pub grid: Vec<Vec<i32>>,
    pub hole: (i32, i32),
    pub finalGrid: Vec<Vec<i32>>,
    pub heuristic: i32,
    pub mixValue: i32,
    pub shortestPath: String
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
impl State
{
    pub fn new() -> State
    {
        State
        {
            size: 0usize,
            grid: vec![vec![0i32; 1]; 1],
            hole: (-1i32, -1i32),
            finalGrid: vec![vec![0i32; 1]; 1],
            heuristic: 0i32,
            mixValue: 0i32,
            shortestPath: String::from("")
        }
    }

    pub fn init(&mut self, size: usize, grid: Vec<Vec<i32>>)
    {
        self.size = size;
        self.grid = grid.clone();
        for y in 0..self.size
        {
            for x in 0..self.size
            {
                if self.grid[y][x] == 0
                {
                    self.hole.0 = x as i32;
                    self.hole.1 = y as i32;
                }
            }
        }
        self.finalGrid = makeFinalGrid(self.size);
        self.setMixValue();
        self.shortestPath = String::from("");
    }

    pub fn print(&self)
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

    pub fn getFinalCoord(&self, value: i32) -> i32
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

    pub fn swap(&self, action: char) -> State
    {
        let mut newState = State::new();
        let mut x = 0;
        let mut y = 0;

        newState.init(self.size, self.grid.clone());
        if (action == 'E' && self.hole.0 < self.size as i32 - 1) || (action == 'W' && self.hole.0 > 0)
        {
            x = if action == 'E' { 1 } else { -1 };
        }
        if (action == 'S' && self.hole.1 < self.size as i32 - 1) || (action == 'N' && self.hole.1 > 0)
        {
            y = if action == 'S' { 1 } else { -1 };
        }
        newState.grid[self.hole.1 as usize][self.hole.0 as usize] = newState.grid[(self.hole.1 + y) as usize][(self.hole.0 + x) as usize];
        newState.grid[(self.hole.1 + y) as usize][(self.hole.0 + x) as usize] = 0;
        newState.hole.0 += x;
        newState.hole.1 += y;

        newState
    }

    pub fn setMixValue(&mut self)
    {
        let mut value = 0i32;

        for y in 0..self.size
        {
            for x in 0..self.size
            {
                if self.grid[y][x] != self.finalGrid[y][x]
                {
                    let temp = self.getFinalCoord((y * self.size + x) as i32);
                    value += i32::abs(temp % self.size as i32 - x as i32) + i32::abs(temp / self.size as i32 - y as i32);
                }
            }
        }
        self.heuristic = value;
    }
}
