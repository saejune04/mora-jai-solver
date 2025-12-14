use mora_jai_solver::{Solver, TileBox};
use mora_jai_solver::Color::{Yellow, Grey, Red, White, Orange, Pink, Purple, Black, Green, Blue};
fn main() {
    let data = [
        Pink, Black, Pink,
        Orange, Grey, Orange,
        Pink, Red, Pink,
    ];

    // Define what the *goal corner colors* are.
    let solution_colors = [Orange, Red, Red, Orange];

    // Construct the TileBox
    let tile_box = TileBox::new(Some(data), solution_colors);

    let mut solver = Solver::new(tile_box);

    println!("{:?}", solver.solve());
}
