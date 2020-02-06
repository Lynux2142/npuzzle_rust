#[derive(Debug)]
pub struct Map
{
    width: usize,
    height: usize,
    grid: Vec<i32>, // ptr to array of i32
    hole: usize,
    heuristic_value: i32,
    cost: i32// heuristic + chemins actuel
}

impl Map
{
    pub fn new() -> Map
    {
        Map
        {
            // comment on fait pour gerer les usize, on laisse a 0 ?
            width: 0,
            height: 0,
            grid: Vec::new(),
            hole: 0,
            heuristic_value: -1,
            cost: -1
        }
    }
}
