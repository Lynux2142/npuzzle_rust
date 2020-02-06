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

    // new from_map ()
}
