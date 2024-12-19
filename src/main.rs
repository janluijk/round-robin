#[derive(Debug, PartialEq, Clone, Copy)]
struct Match {
    left: i32,
    right: i32,
}

impl Match {
    fn new(left: i32, right: i32) -> Self {
        Match { left, right }
    }
}

struct Matchup {
    data: [[Match; 6]; 6],
}

impl Matchup {
    fn new(default_value: Match) -> Self {
        Self {
            data: [[default_value; 6]; 6],
        }
    }
    fn reset(&mut self) {
        for row in &mut self.data {
            for entry in row.iter_mut() {
                entry.left = 0;
                entry.right = 0;
            }
        }
    }

    fn print(&self) {
        for row in self.data.iter() {
            for entry in row.iter() {
                print!("({}) ({}), ", entry.left, entry.right)
            }
            println!();
        }
    }

    fn check_row(&self) -> bool {
        for row in &self.data {
            let mut seen = [false; 13];

            for entry in row {
                if entry.left < 1 || entry.left > 12 {
                    return false;
                }

                if entry.right < 1 || entry.right > 12 {
                    return false;
                }

                seen[entry.left as usize] = true;
                seen[entry.right as usize] = true;
            }

            let no_duplicates_or_missing = seen[1..=12].iter().all(|&x| x);
            if !no_duplicates_or_missing {
                return false;
            }
        }

        true
    }

    fn check_column(&self) -> bool {
        for col in 0..6 {
            let mut seen = [false; 13];

            for row in 0..6 {
                let entry = self.data[row][col];

                if entry.left < 1 || entry.left > 12 {
                    return false;
                }

                if entry.right < 1 || entry.right > 12 {
                    return false;
                }

                seen[entry.left as usize] = true;
                seen[entry.right as usize] = true;
            }

            let no_duplicates_or_missing = seen[1..=12].iter().all(|&x| x);
            if !no_duplicates_or_missing {
                return false;
            }
        }
        true
    }

    fn check_unique_pairs(&self) -> bool {
        let mut seen_pairs = std::collections::HashSet::new();

        for row in &self.data {
            for entry in row {
                if !seen_pairs.insert((entry.left, entry.right)) {
                    return false;
                }
            }
        }

        true
    }

    fn check_matchup(&self) -> bool {
        self.check_row() && self.check_column() && self.check_unique_pairs()
    }

    fn find_solution(&mut self) -> bool {
        let mut pairs = [false; 144];
        self.solve(0, 0, &mut pairs);
        self.print();
        self.reset();
        pairs[Self::pair_index(1, 2)] = false;
        pairs[Self::pair_index(3, 4)] = false;
        pairs[Self::pair_index(5, 6)] = false;
        pairs[Self::pair_index(7, 8)] = false;
        pairs[Self::pair_index(9, 10)] = false;
        pairs[Self::pair_index(11, 12)] = false;
        self.print();
        self.solve(0, 0, &mut pairs)
    }

    fn pair_index(left: i32, right: i32) -> usize {
        let (min, max) = (left.min(right), left.max(right));
        (min - 1) as usize * 12 + (max - min - 1) as usize
    }

    fn solve(&mut self, row: usize, col: usize, used_pairs: &mut [bool; 144]) -> bool {
        if col == 6 {
            return if row == 5 {
                true
            } else {
                self.solve(row + 1, 0, used_pairs)
            };
        }

        let mut available: u32 = 0b1111111111110;

        for r in 0..6 {
            for c in 0..6 {
                let entry = self.data[r][c];

                if entry.left == 0 || entry.right == 0 {
                    break;
                }

                available &= !(1 << entry.left);
                available &= !(1 << entry.right);
            }
        }

        for left_value in 1..=12 {
            if available & (1 << left_value) {
                available &= !(1 << left_value); // Mark left_value as unavailable

                let original_available = available;

                for right_value in 1..=12 {
                    if available & (1 << right_value) {
                        available &= !(1 << right_value); // Mark right_value as unavailable

                        let pair_index = Self::pair_index(left_value, right_value);

                        if used_pairs[pair_index] {
                            continue;
                        }

                        self.data[row][col] = Match::new(left_value, right_value);
                        used_pairs[pair_index] = true;

                        if self.solve(row, col + 1, used_pairs) {
                            return true;
                        }

                        self.data[row][col] = Match::new(0, 0);
                        used_pairs[pair_index] = false;
                    }
                }

                available = original_available;
            }
        }
        false
    }
}

fn main() {
    let default_value = Match::new(0, 0);
    let mut matchup = Matchup::new(default_value);

    if matchup.find_solution() {
        println!("Solution found");

        for row in matchup.data.iter() {
            for entry in row.iter() {
                print!("({}) ({}), ", entry.left, entry.right)
            }
            println!();
        }
    } else {
        println!("No solution found.");
    }

    if matchup.check_matchup() {
        println!("All criteria passed!");
    } else {
        println!("Some criteria failed.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_type_creation() {
        let match1 = Match::new(1, 2);
        assert_eq!(match1.left, 1);
        assert_eq!(match1.right, 2);
    }

    #[test]
    fn test_matchup_creation() {
        let default_value = Match::new(0, 0);
        let matchup = Matchup::new(default_value);

        for row in &matchup.data {
            for entry in row {
                assert_eq!(*entry, default_value);
            }
        }
    }
}
