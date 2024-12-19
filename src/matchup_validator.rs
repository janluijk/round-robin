use crate::matchup::Matchup;

pub struct MatchupValidator<'a> {
    matchup: &'a Matchup,
}

impl<'a> MatchupValidator<'a> {
    pub fn new(matchup: &'a Matchup) -> Self {
        MatchupValidator { matchup }
    }

    pub fn validate_matchup(&self) -> bool {
        let rows = self.matchup.rows;
        let cols = self.matchup.cols;

        for row in 0..rows {
            for col in 0..cols {
                let mut available = self.get_available_bitmask(rows);

                if !self.is_valid_for_cell(row, col, &mut available) {
                    return false;
                }
            }
        }
        true
    }

    fn get_available_bitmask(&self, rows: usize) -> u32 {
        (1u32 << (rows + 1)) - 1
    }

    fn is_valid_for_cell(&self, row: usize, col: usize, available: &mut u32) -> bool {
        let rows = self.matchup.rows;
        let cols = self.matchup.cols;

        let mut range = 0..cols;
        let second_half = row > cols;

        if second_half {
            range = cols..rows
        }

        for c in 0..cols {
            let entry = self.matchup.data[row][c];

            if entry.left != 0 || entry.right != 0 {
                self.set_value(entry.left, available);
                self.set_value(entry.right, available);
            }
        }

        for r in range {
            let entry = self.matchup.data[r][col];

            if entry.left != 0 || entry.right != 0 {
                self.set_value(entry.left, available);
                self.set_value(entry.right, available);
            }
        }

        let not_all_numbers_used = *available != 0;
        if not_all_numbers_used {
            return false;
        }
        true
    }

    fn set_value(&self, value: i32, available: &mut u32) {
        if value != 0 {
            *available &= !(1 << value);
        }
    }
}
