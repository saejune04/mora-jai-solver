use crate::Color;
use std::collections::HashMap;

const MIDDLE_INDEX: usize = 4;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct TileBox {
    // TileBox is a 3x3 grid of colors represented by a 1D array
    // The array is indexed row-wise from top-left to bottom-right
    data: [Color; 9],
    // The solution corner colors in order: top-left, top-right, bottom-left, bottom-right
    solution_colors: [Color; 4],
}

impl TileBox {
    pub fn new(data: Option<[Color; 9]>, solution_colors: [Color; 4]) -> Self {
        TileBox {
            data: data.unwrap_or([Color::Grey; 9]),
            solution_colors,
        }
    }
    
    pub fn simulate_click(&mut self, index: usize) {
        // Index should be between 0 and 8 inclusive
        if index > 8 {
            panic!("Index out of bounds");
        }

        let clicked_color = self.data[index];

        self.match_click_from_color(clicked_color, index);
    }

    // Checks if all 4 corner tiles match the solution color
    pub fn is_solved(&self) -> bool {
        return self.data[0] == self.solution_colors[0] &&
               self.data[2] == self.solution_colors[1] &&
               self.data[6] == self.solution_colors[2] &&
               self.data[8] == self.solution_colors[3];
    }

    fn match_click_from_color(&mut self, color: Color, index: usize) {
        match color {
            Color::Yellow => self.click_yellow(index),
            Color::Grey => self.click_grey(index),
            Color::Red => self.click_red(index),
            Color::White => self.click_white(index),
            Color::Orange => self.click_orange(index),
            Color::Pink => self.click_pink(index),
            Color::Purple => self.click_purple(index),
            Color::Black => self.click_black(index),
            Color::Green => self.click_green(index),
            Color::Blue => self.click_blue(index),
        }
    } 

    fn click_yellow(&mut self, index: usize) {
        // Yellow moves the yellow tile up, doing nothing at the top row
        let row = index / 3;
        let col = index % 3;
        if row == 0 {
            return;
        }
        let temp = self.data[(row - 1) * 3 + col];
        self.data[(row - 1) * 3 + col] = self.data[index];
        self.data[index] = temp;
    }

    fn click_grey(&mut self, _index: usize) {
        // Grey does nothing
        return;
    }

    fn click_red(&mut self, _index: usize) {
        // Red turns all black tiles into red ones and all white tiles into black ones
        for color in self.data.iter_mut() {
            if *color == Color::Black {
                *color = Color::Red;
            }
        }

        for color in self.data.iter_mut() {
            if *color == Color::White {
                *color = Color::Black;
            }
        }
        
    }

    fn click_white(&mut self, index: usize) {
        // Transforms all orthogonally adjacent tiles of the same color and this tile to Grey,
        // Turns any orthogonally adjacent grey tiles to this tile's color.
        // Note: in the case of blue, it will use BLUE rather than WHITE
        let row = index / 3;
        let col = index % 3;
        let curr_color = self.data[index];

        for i in 0..3 {
            for j in 0..3 {
                if (row as isize - i as isize).abs() + (col as isize - j as isize).abs() <= 1 {
                    if self.data[i * 3 + j] == curr_color {
                        self.data[i * 3 + j] = Color::Grey;
                    } else if self.data[i * 3 + j] == Color::Grey {
                        self.data[i * 3 + j] = curr_color;
                    }
                }
            }
        }
    }

    fn click_orange(&mut self, index: usize) {
        // If there is a majority of a color adjacent to the orange tile
        // then convert to that color
        let row = index / 3;
        let col = index % 3;

        // Count adjacent colors
        let mut color_count: HashMap<Color, usize> = HashMap::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in directions.iter() {
            let new_row = row as isize + dy;
            let new_col = col as isize + dx;
            if new_row < 0 || new_row > 2 || new_col < 0 || new_col > 2 {
                continue;
            }
            let neighbor_index = (new_row as usize) * 3 + (new_col as usize);
            
            // Count colors of neighbors
            let count = color_count.entry(self.data[neighbor_index]).or_insert(0);
            *count += 1;
        }

        let mut majority_color: Option<Color> = None;
        let mut sole_majority = true;
        let mut max_count = 0;
        for (color, count) in color_count.iter() {
            if *count > max_count {
                max_count = *count;
                majority_color = Some(*color);
                sole_majority = true;
            } else if *count == max_count {
                sole_majority = false;
            }
        }

        // If there is a sole majority, change the tile to that color. Else do nothing.
        if sole_majority && max_count > 0 {
            self.data[index] = majority_color.unwrap();
        }
    }
    
    fn click_pink(&mut self, index: usize) { 
        // Pink rotates all adjacent tiles clockwise, teleporting to the next 
        // valid position if the tile were to go off the edge
        let row = index / 3;
        let col = index % 3;
    
        let mut rotation_indices: Vec<usize> = Vec::new();
        let directions = [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
        for (dy, dx) in directions.iter() {
            let new_col = col as isize + dx;
            let new_row = row as isize + dy;

            if new_row < 0 || new_row > 2 || new_col < 0 || new_col > 2 {
                continue;
            }
            rotation_indices.push((new_row as usize) * 3 + (new_col as usize));
        }
        if rotation_indices.len() == 0 {
            return;
        }

        let temp = self.data[rotation_indices[rotation_indices.len() - 1]];
        for i in (1..rotation_indices.len()).rev() {
            self.data[rotation_indices[i]] = self.data[rotation_indices[i - 1]];
        }
        self.data[rotation_indices[0]] = temp;
    }

    fn click_purple(&mut self, index: usize) {
        // Purple moves the tile down, doing nothing at the bottom row
        let row = index / 3;
        let col = index % 3;

        if row == 2 {
            return;
        }

        let temp = self.data[(row + 1) * 3 + col];
        self.data[(row + 1) * 3 + col] = self.data[index];
        self.data[index] = temp;
    }

    fn click_black(&mut self, index: usize) {
        // Horizontal rotation: shifts entire row to the right, and the right-most tile to the left
        let row = index / 3;

        let temp = self.data[row * 3 + 2];
        for c in (1..3).rev() {
            self.data[row * 3 + c] = self.data[row * 3 + c - 1];
        }
        self.data[row * 3] = temp;
    }

    fn click_green(&mut self, index: usize) {
        // Swaps with the opposite tile on the far side of the box; no effect in the center
        let row = index / 3;
        let col = index % 3;

        let opp_row = 2 - row;
        let opp_col = 2 - col;
        let temp = self.data[opp_row * 3 + opp_col];
        self.data[opp_row * 3 + opp_col] = self.data[index];
        self.data[index] = temp;
    }

    fn click_blue(&mut self, index: usize) {
        // Behaves the same way as the center tile; no effect if the center tile is blue.
        if self.data[MIDDLE_INDEX] == Color::Blue {
            return;
        }
        self.match_click_from_color(self.data[MIDDLE_INDEX], index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}