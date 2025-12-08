use mora_jai_solver::{Color, Solver, TileBox};
fn main() {
    let data = [
        Color::Orange, Color::Grey, Color::Blue,
        Color::Blue, Color::Orange, Color::Black,
        Color::Yellow, Color::Grey, Color::Green,
    ];

    // Define what the *goal corner color* is
    let solution_color = Color::Black;

    // Construct the TileBox
    let tile_box = TileBox::new(Some(data), solution_color);

    let mut solver = Solver::new(tile_box);

    println!("{:?}", solver.solve());
}
