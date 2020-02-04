#[allow(non_snake_case)]
pub struct State
{
    pub size: usize,
    pub grid: Vec<Vec<i32>>,
    pub hole: (i32, i32),
    pub finalGrid: Vec<Vec<i32>>
}

#[allow(non_snake_case)]
impl State
{
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
