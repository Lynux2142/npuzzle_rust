use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
#[derive(Debug)]
pub struct Map
{
    pub size: usize,
    pub width: usize,
    pub height: usize,
    pub grid: Vec<i32>, // ptr to array of i32
    pub hole: usize,
    pub heuristic_value: i32,
    pub cost: i32, // heuristic + chemins actuel
    pub shortest_path: String,
}

impl Map
{
    pub fn new() -> Map
    {
        Map
        {
            size: 0,
            width: 0,
            height: 0,
            grid: Vec::new(),
            hole: 0,
            heuristic_value: i32::max_value(),
            cost: -1,
            shortest_path: String::new()
        }
    }

    pub fn print(&self)
    {
        let mut hole_size = 0;
        let mut value_size;
        let mut tmp = (self.size) as i32;

        while tmp >= 10 { hole_size += 1; tmp /= 10; }

        for i in 0..self.size
        {
            value_size = 0;
            if self.grid[i] != 0
            {
                tmp = self.grid[i];
                while tmp >= 10 { value_size += 1; tmp /= 10; }
            }
            for _ in value_size..hole_size { print!(" "); }
            if self.grid[i] != 0 { print!("{}", self.grid[i]); }
            else { print!(" "); }
            if (i + 1) % self.width != 0 { print!(" "); } else { println!(); }
        }
    }

    pub fn get_key(&self) -> String { format!("{:?}", self.grid) }
    // new from_map ()
}
/*
    pub size: usize,
    pub width: usize,
    pub height: usize,
    pub grid: Vec<i32>, // ptr to array of i32
    pub hole: usize,
    pub heuristic_value: i32,
    pub cost: i32, // heuristic + chemins actuel
    pub shortest_path: String
*/

impl Clone for Map
{
    fn clone(&self) -> Map
    {
        Map
        {
            size: self.size,
            width: self.width,
            height: self.height,
            grid: self.grid.clone(),
            hole: self.hole,
            heuristic_value: self.heuristic_value,
            cost: self.cost,
            shortest_path: self.shortest_path.clone()
        }
    }
}

impl Ord for Map
{
    fn cmp(&self, other: &Map) -> Ordering
    {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Map
{
    fn partial_cmp(&self, other: &Map) -> Option<Ordering>
    {
        Some(other.cost.cmp(&self.cost))
    }
}
