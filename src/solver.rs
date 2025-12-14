use crate::tilebox::TileBox;
use std::collections::HashMap;

pub struct Solver {
    tile_box: TileBox,
}

impl Solver {
    pub fn new(tile_box: TileBox) -> Self {
        Solver { tile_box }
    }

    // Solves the TileBox puzzle and returns the sequence of clicks in numpad notation
    pub fn solve(&mut self) -> Vec<usize> {
        // TODO: multithread and caching
        match self.bfs() {
            Some(solution) => Self::map_sol_to_numpad_notation(solution),
            None => vec![],
        }
    }

    // Maps internal 0-8 indexing to numpad notation
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
                    continue; // Skip branches with moves that do not change the state of the box
                }

                if !visited.contains(&new_box) {
                    visited.insert(new_box.clone());
                    let mut new_path = path.clone();
                    new_path.push(i);
                    queue.push_back((new_box, new_path));
                }
            }
        }
        println!("No solution found");
        None // No solution found
    }
}