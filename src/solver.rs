use crate::tilebox::TileBox;
use threadpool::Builder;
use crossbeam::channel;

pub struct Solver {
    tile_box: TileBox,
}

impl Solver {
    pub fn new(tile_box: TileBox) -> Self {
        Solver { tile_box }
    }

    pub fn solve(&mut self, max_depth: usize) -> Vec<usize> {
        // TODO: multithread and caching
        match self.brute_force_multithreaded(max_depth) {
            Some(solution) => solution,
            None => vec![],
        }
    }

    fn brute_force_multithreaded(&mut self, max_depth: usize) -> Option<Vec<usize>> {
        let pool = Builder::new().build();
        println!("Starting solver with {} threads", pool.max_count());

        let (result_tx, result_rx) = channel::unbounded::<Vec<usize>>();

        let mut curr_depth = 1usize;

        // Clone the initial box state,
        // Formulate the next set of moves for the current depth,
        // Test the moves on the cloned box and see if we get a solved state
        // If we tested all moves at the current depth without success, increase depth and repeat
        while curr_depth <= max_depth {
            let mut moves = vec![0usize; curr_depth];

            loop {
                let mut moves_clone = moves.clone();
                let result_tx_clone = result_tx.clone();
                let box_clone = self.tile_box.clone();
                pool.execute(move || {
                    let current_box = box_clone.clone();
                    if Solver::test_moves_on_box(&Solver { tile_box: box_clone.clone() }, current_box, &mut moves_clone) {
                        result_tx_clone.send(moves_clone).unwrap();
                    }
                });

                // Get the next set of moves to test
                if !Solver::increment_moves(&mut moves, 9) {
                    break;
                } 
            } 
            pool.join();
            // Check for results
            if let Ok(solution) = result_rx.try_recv() {
                return Some(solution);
            }

            curr_depth += 1;
            println!("Increasing depth to {}", curr_depth);

        }
        None
    }

    // Give a TileBox and a set of moves, to perform, simulates the moves on the box
    // and returns whether or not the box is solved
    fn test_moves_on_box(&self, mut current_box: TileBox, moves: &mut Vec<usize>) -> bool {
        for box_move in moves.iter() {
            current_box.simulate_click(*box_move);
        }
        if current_box.is_solved() {
            return true;
        }
        false
    }

    // Increments the moves vector to the next combination of moves
    // Returns false if all combinations have been exhausted for the given move length
    // base is the number of possible moves (9 for a 3x3 grid)
    fn increment_moves(moves: &mut [usize], base: usize) -> bool {
        for i in (0..moves.len()).rev() {
            if moves[i] + 1 < base {
                moves[i] += 1;
                // println!("Next moves: {:?}", moves);
                return true;
            } else {
                // carry
                moves[i] = 0;
            }
        }
        // wrapped past the last combination
        false
    }
}