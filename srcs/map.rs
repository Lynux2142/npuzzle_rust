#[derive(Debug)]
pub struct Map
{
    pub size: usize,
    pub width: usize,
    pub height: usize,
    pub grid: Vec<i32>, // ptr to array of i32
    pub hole: i32,
    pub heuristic_value: i32,
    pub cost: i32// heuristic + chemins actuel
}

impl Map
{
    pub fn new() -> Map
    {
        Map
        {
            // comment on fait pour gerer les usize, on laisse a 0 ?
            size: 0,
            width: 0,
            height: 0,
            grid: Vec::new(),
            hole: 0,
            heuristic_value: -1,
            cost: -1
        }
    }

    pub fn print(&self)
    {
        let mut holeSize = 0;
        let mut valueSize;
        let mut tmp = (self.size) as i32;

        while tmp >= 10 { holeSize += 1; tmp /= 10; }

        for i in 0..self.size
        {
            valueSize = 0;
            if self.grid[i] != 0
            {
                tmp = self.grid[i];
                while tmp >= 10 { valueSize += 1; tmp /= 10; }
            }
            for _ in valueSize..holeSize { print!(" "); }
            print!("{}", self.grid[i]);
            if (i + 1) % self.width != 0 { print!(" "); } else { println!(); }
        }
    }

    // new from_map ()
}
