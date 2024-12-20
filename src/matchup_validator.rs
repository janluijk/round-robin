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

        let mut matches = vec![true; rows * cols];

        for row in 0..rows {
            for col in 0..cols {
                if !self.is_valid_for_cell(row, col, &mut matches) {
                    return false;
                }
            }
        }

        self.all_matches_used(&matches)
    }

    fn is_valid_for_cell(&self, row: usize, col: usize, matches: &mut [bool]) -> bool {
        let entry = self.matchup.data[row][col];

        let matchnr = self.data_to_match(entry.left, entry.right);
        if !matches[matchnr] {
            return false;
        }

        matches[matchnr] = false;
        true
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

    fn all_matches_used(&self, matches: &[bool]) -> bool {
        matches.iter().all(|&value| !value)
    }
}
