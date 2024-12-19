use crate::matchup::Matchup;

enum Method {
    PerCell,
    PerPair,
}

pub struct MatchupSolver<'a> {
    matchup: &'a Matchup,
    method: Method,
}

impl<'a> MatchupSolver<'a> {
    pub fn new(matchup: &'a Matchup) -> Self {
        let method = Method::PerPair;
        MatchupSolver { matchup, method }
    }

    pub fn solve(&mut self) -> bool {
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
        true
    }

    fn solve_pairs(&mut self) -> bool {
        true
    }
}
