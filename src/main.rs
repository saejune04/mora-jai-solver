use mora_jai_solver::{Color, Solver, TileBox};
fn main() {
    let max_depth = 20;
    let data = [
        Color::Black, Color::Black, Color::Black,
        Color::Green, Color::Black, Color::Grey,
        Color::Grey, Color::Grey, Color::Purple,
    ];

    // Define what the *goal corner color* is
    let solution_color = Color::Black;

    // Construct the TileBox
    let tile_box = TileBox::new(Some(data), solution_color);

    let mut solver = Solver::new(tile_box);

    println!("{:?}", solver.solve(max_depth));
}
