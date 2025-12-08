use crate::tilebox::TileBox;
use rayon::prelude::*;
use std::sync::{ Arc, atomic::{ AtomicBool, Ordering } };
use std::collections::HashMap;

pub struct Solver {
    tile_box: TileBox,
}

impl Solver {
    pub fn new(tile_box: TileBox) -> Self {
        Solver { tile_box }
    }

    pub fn solve(&mut self, _max_depth: usize) -> Vec<usize> {
        // TODO: multithread and caching
        // match self.brute_force_multithreaded(max_depth) {
        //     Some(solution) => solution,
        //     None => vec![],
        // }
        match self.bfs() {
            Some(solution) => Self::map_sol_to_numpad_notation(solution),
            None => vec![],
        }
    }

    fn map_sol_to_numpad_notation(solution: Vec<usize>) -> Vec<usize> {
        let mut mapping = HashMap::new();
        mapping.insert(0, 7);
        mapping.insert(1, 8);
        mapping.insert(2, 9);
        mapping.insert(3, 4);
        mapping.insert(4, 5);
        mapping.insert(5, 6);
        mapping.insert(6, 1);
        mapping.insert(7, 2);
        mapping.insert(8, 3);
        solution.iter().map(|&idx| *mapping.get(&idx).unwrap()).collect()
    }

    fn bfs(&self) -> Option<Vec<usize>> {
        let root = self.tile_box.clone();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((root, vec![]));
        while let Some((current_box, path)) = queue.pop_front() {
            if current_box.is_solved() {
                return Some(path);
            }

            for i in 0..9 {
                let mut new_box = current_box.clone();
                new_box.simulate_click(i);
                if new_box == current_box {
                    continue; // Skip no-op moves
                }

                if !visited.contains(&new_box) {
                    visited.insert(new_box.clone());
                    let mut new_path = path.clone();
                    new_path.push(i);
                    queue.push_back((new_box, new_path));
                }
            }
        }
        None
    }

    fn brute_force_multithreaded(&mut self, max_depth: usize) -> Option<Vec<usize>> {
        // Use an atomic flag to signal all threads to stop once a solution is found
        let found_flag = Arc::new(AtomicBool::new(false));

        // Iterative Deepening
        for depth in 1..=max_depth {
            println!("Trying depth {}", depth);

            // Parallelize over the initial move
            let result = (0..9).into_par_iter().find_map_any(|first_move| {
                if found_flag.load(Ordering::Relaxed) {
                    return None;
                }

                let mut moves = vec![0; depth];
                moves[0] = first_move;

                loop {
                    if found_flag.load(Ordering::Relaxed) {
                        return None;
                    }

                    let current_box = self.tile_box.clone();
                    if Solver::test_moves_on_box(current_box, &mut moves) {
                        found_flag.store(true, Ordering::Relaxed);
                        return Some(moves.clone());
                    }

                    if !Solver::increment_moves(&mut moves[1..], 9) {
                        break;
                    }
                }
                None
            });
            if let Some(solution) = result {
                return Some(solution);
            }
        }
        None
    }

    // Give a TileBox and a set of moves, to perform, simulates the moves on the box
    // and returns whether or not the box is solved
    fn test_moves_on_box(mut current_box: TileBox, moves: &mut Vec<usize>) -> bool {
        for box_move in moves.iter() {
            let old_box = current_box.clone();
            current_box.simulate_click(*box_move);

            // Prune this set of moves if there is a move that does nothing
            if old_box == current_box {
                return false;
            }
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