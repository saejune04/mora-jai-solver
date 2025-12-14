use mora_jai_solver::{Solver, TileBox};
use mora_jai_solver::Color::{Yellow, Grey, Red, White, Orange, Pink, Purple, Black, Green, Blue};
fn main() {
    let data = [
    Pink, White, Pink,
    Blue, Grey, Blue,
    Pink, Red, Pink
    ];

    // Define what the *goal corner colors* are.
    let solution_colors = [Red, Red, Red, Red];

    // Construct the TileBox
    let tile_box = TileBox::new(Some(data), solution_colors);

    let mut solver = Solver::new(tile_box);

    println!("{:?}", solver.solve());
}
