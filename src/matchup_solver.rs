use crate::matchup::Matchup;

enum Method {
    PerCell,
    PerPair,
}

pub struct MatchupSolver<'a> {
    matchup: &'a mut Matchup,
    method: Method,
}

impl<'a> MatchupSolver<'a> {
    pub fn new(matchup: &'a mut Matchup) -> Self {
        let method = Method::PerCell;
        MatchupSolver { matchup, method }
    }

    pub fn solve(&mut self) -> bool {
        self.solve_cells();
        match self.method {
            Method::PerPair => self.solve_pairs(),
            Method::PerCell => self.solve_cells(),
        }
    }

    pub fn _use_pair_method(&mut self) {
        self.method = Method::PerPair;
    }

    pub fn use_cell_method(&mut self) {
        self.method = Method::PerPair;
    }

    fn solve_cells(&mut self) -> bool {
        let rows = self.matchup.rows;
        let cols = self.matchup.cols;
        let max_cell_nr = rows * cols;

        let mut matches = vec![true; max_cell_nr];
        let mut cellnr = 0;
        let mut start_from_match = 0;

        while cellnr < max_cell_nr {
            if self.solve_cell(&mut matches, cellnr, start_from_match) {
                cellnr += 1;
                start_from_match = 0;
            } else {
                cellnr -= 1;
                let previous_match = self.prev_cell_to_match(&mut matches, cellnr);
                start_from_match = previous_match;
            }
        }
        true
    }

    fn solve_cell(&mut self, matches: &mut [bool], cellnr: usize, start_match: usize) -> bool {
        let row = cellnr / self.matchup.cols;
        let col = cellnr % self.matchup.cols; // probably optimised by compiler

        let available = self.get_available(row, col);
        if let Some(new_match) = self.get_first_match(matches, available, start_match) {
            let (left, right) = self.match_to_data(new_match);
            let cell = &mut self.matchup.data[row][col];
            cell.left = left;
            cell.right = right;
            true
        } else {
            false
        }
    }

    fn get_available(&self, curr_row: usize, curr_col: usize) -> u32 {
        let mut available = self.get_available_bitmask();

        for c in 0..curr_col {
            let cell = self.matchup.data[curr_row][c];

            self.set_value(cell.left, &mut available);
            self.set_value(cell.right, &mut available);
        }

        let mut range = 0..curr_row;
        let second_half = curr_row >= self.matchup.cols;

        if second_half {
            range = self.matchup.cols..curr_row;
        }

        for r in range {
            let cell = self.matchup.data[r][curr_col];

            self.set_value(cell.left, &mut available);
            self.set_value(cell.right, &mut available);
        }

        available
    }

    #[inline]
    fn get_available_bitmask(&self) -> u32 {
        ((1u32 << (self.matchup.rows + 2)) - 1) & !1
    }

    #[inline]
    fn set_value(&self, value: usize, available: &mut u32) {
        if value != 0 {
            *available &= !(1 << value);
        }
    }

    fn prev_cell_to_match(&self, matches: &mut [bool], cellnr: usize) -> usize {
        let row = cellnr / self.matchup.cols;
        let col = cellnr % self.matchup.cols;
        let mut cell = self.matchup.data[row][col];

        let prev_cell_match = self.data_to_match(cell.left, cell.right);
        let start_match = prev_cell_match + 1;

        cell.left = 0;
        cell.right = 0;

        matches[prev_cell_match] = true;

        start_match
    }

    fn data_to_match(&self, left: usize, right: usize) -> usize {
        let start = self.matchup.rows;

        let count = left - 1;
        let mut sum = 0;

        for i in 0..count {
            sum += start - i;
        }

        let diff = right - left;
        sum += diff - 1;
        sum
    }

    fn match_to_data(&self, mut number: usize) -> (usize, usize) {
        let mut subtract_value = self.matchup.rows;
        let mut iterations = 0;

        while number >= subtract_value {
            number -= subtract_value;
            iterations += 1;
            subtract_value -= 1;
        }

        let left = iterations + 1;
        let right = left + number + 1;

        (left, right)
    }

    fn get_first_match(
        &self,
        matches: &mut [bool],
        available: u32,
        start_match: usize,
    ) -> Option<usize> {
        (start_match..self.matchup.rows * self.matchup.cols).find(|&matchnr| {
            if matches[matchnr] {
                let (left, right) = self.match_to_data(matchnr);

                let left_available = (available & (1 << left)) != 0;
                let right_available = (available & (1 << right)) != 0;

                if left_available && right_available {
                    matches[matchnr] = false;
                    return true;
                }
            }
            false
        })
    }

    fn solve_pairs(&mut self) -> bool {
        true
    }
}
