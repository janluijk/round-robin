mod matchup;
mod matchup_solver;
mod matchup_validator;

use crate::matchup::{Match, Matchup};
use crate::matchup_solver::MatchupSolver;
use crate::matchup_validator::MatchupValidator;

//fn populate_data(matchup: &mut Matchup) {
//    matchup.data[0][0] = Match::new(1, 2);
//    matchup.data[0][1] = Match::new(3, 4);
//    matchup.data[0][2] = Match::new(5, 6);
//    matchup.data[0][3] = Match::new(7, 8);
//    matchup.data[0][4] = Match::new(9, 10);
//
//    matchup.data[1][0] = Match::new(1, 2);
//    matchup.data[1][1] = Match::new(3, 4);
//    matchup.data[1][2] = Match::new(5, 6);
//    matchup.data[1][3] = Match::new(7, 8);
//    matchup.data[1][4] = Match::new(9, 10);
//
//    matchup.data[2][0] = Match::new(11, 12);
//    matchup.data[2][1] = Match::new(13, 14);
//    matchup.data[2][2] = Match::new(15, 16);
//    matchup.data[2][3] = Match::new(17, 18);
//    matchup.data[2][4] = Match::new(19, 20);
//}

fn main() {
    let size = 4;
    let default_match = Match::new(0, 0);
    let mut matchup = Matchup::new(size, default_match);

    {
        let mut solver = MatchupSolver::new(&mut matchup);
        solver.use_cell_method();
        solver.solve();
    }

    {
        let validator = MatchupValidator::new(&matchup);
        validator.validate_matchup();
    }
    matchup.print();
    matchup.clean();
}
