use crate::matchup::Matchup;

enum Method {
    PerCell,
    PerPair,
}

pub struct MatchupSolver<'a> {
    matchup: &'a mut Matchup,
    method: Method,
    size: usize,
}

impl<'a> MatchupSolver<'a> {
    pub fn new(matchup: &'a mut Matchup) -> Self {
        let method = Method::PerCell;
        let size = matchup.cols;
        MatchupSolver {
            matchup,
            method,
            size,
        }
    }

    pub fn solve(&mut self) -> bool {
        self.solve_cells();
        match self.method {
            Method::PerPair => self.solve_pairs(),
            Method::PerCell => self.solve_cells(),
        }
    }

    pub fn use_pair_method(&mut self) {
        self.method = Method::PerPair;
    }

    pub fn use_cell_method(&mut self) {
        self.method = Method::PerPair;
    }

    fn solve_cells(&mut self) -> bool {
        let rows = self.matchup.rows;
        let cols = self.matchup.cols;

        let mut cellnr = 0;
        let mut start_from_match = 0;
        let max_cell_nr = rows * cols;
        let max_matches = rows * cols;
        let mut matches = vec![true; max_matches];

        while cellnr < max_cell_nr {
            let match_found = self.solve_cell(&mut matches, cellnr, start_from_match);
            let no_match_found = match_found == -1;

            if no_match_found {
                cellnr -= 1;
                let previous_match = self.prev_cell_to_match(&mut matches, cellnr);
                start_from_match = previous_match;
            } else {
                cellnr += 1;
                start_from_match = 0;
            }

            if cellnr > cols * (rows - 1) {
                self.matchup.print();
            }
        }

        true
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

    fn solve_cell(&mut self, matches: &mut [bool], cellnr: usize, start_match: usize) -> i32 {
        let curr_row = cellnr / self.size;
        let curr_col = cellnr % self.size;
        let available = self.get_available(curr_row, curr_col);
        let new_match = self.get_first_match(matches, available, start_match);

        if new_match >= 0 {
            let data = self.match_to_data(new_match as usize);
            self.matchup.data[curr_row][curr_col].left = data.0;
            self.matchup.data[curr_row][curr_col].right = data.1;
        }

        new_match
    }

    fn get_first_match(&self, matches: &mut [bool], available: u32, start_match: usize) -> i32 {
        let max_matches = self.matchup.rows * self.matchup.cols;

        let mut matchnr = start_match;

        while matchnr < max_matches {
            if !matches[matchnr] {
                matchnr += 1;
                continue;
            }

            let pair = self.match_to_data(matchnr);
            let left = pair.0;
            let right = pair.1;

            let right_available = (available & (1 << right)) != 0;
            let left_available = (available & (1 << left)) != 0;

            if left_available && right_available {
                matches[matchnr] = false;

                return matchnr as i32;
            }

            matchnr += 1;
        }
        -1
    }

    fn get_available(&self, curr_row: usize, curr_col: usize) -> u32 {
        let mut available = self.get_available_bitmask();

        for c in 0..curr_col {
            let entry = self.matchup.data[curr_row][c];

            self.set_value(entry.left, &mut available);
            self.set_value(entry.right, &mut available);
        }

        let mut range = 0..curr_row;
        let second_half = curr_row >= self.matchup.cols;

        if second_half {
            range = self.size..curr_row;
        }

        for r in range {
            let entry = self.matchup.data[r][curr_col];

            self.set_value(entry.left, &mut available);
            self.set_value(entry.right, &mut available);
        }

        available
    }

    fn solve_pairs(&mut self) -> bool {
        true
    }

    fn set_value(&self, value: usize, available: &mut u32) {
        if value != 0 {
            *available &= !(1 << value);
        }
    }

    fn get_available_bitmask(&self) -> u32 {
        ((1u32 << (self.matchup.rows + 2)) - 1) & !1
    }
}
