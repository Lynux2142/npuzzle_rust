mod map;
mod map_procedure;


use map::Map;
use map_procedure::core_swap;

//          5   6   7
//          1   0   3
//          4   2   8

fn main () {
  let mut test = Map::new();

   // travail de parsing a mettre ici
   test.width = 3;
   test.height = 3;
   test.grid = vec![5,6,7,1,0,3,4,2,8];
   test.hole = 4;

   core_swap(&test, 'r');
}
