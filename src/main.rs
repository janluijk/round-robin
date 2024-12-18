use std::collections::HashSet;

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
    data: [[Match; 6]; 12],
}

impl Matchup {
    fn new(default_value: Match) -> Self {
        Self {
            data: [[default_value; 6]; 12],
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
        self.solve(0, 0)
    }

    fn solve(&mut self, row: usize, col: usize) -> bool {
        if col == 6 {
            return if row == 12 {
                true
            } else {
                self.solve(row + 1, 0)
            };
        }

        let mut available = [true; 13];
        let mut used_pairs = HashSet::new();

        if row < 7 {
            for r in 0..6 {
                let left = self.data[r][col].left;
                let right = self.data[r][col].right;

                if left != 0 {
                    available[left as usize] = false;
                }

                if right != 0 {
                    available[right as usize] = false;
                }

                if left != 0 && right != 0 {
                    used_pairs.insert((left.min(right), left.max(right)));
                }
            }
        } else {
            for r in 6..12 {
                let left = self.data[r][col].left;
                let right = self.data[r][col].right;

                if left != 0 {
                    available[left as usize] = false;
                }

                if right != 0 {
                    available[right as usize] = false;
                }

                if left != 0 && right != 0 {
                    used_pairs.insert((left.min(right), left.max(right)));
                }
            }
        }

        for c in 0..6 {
            let left = self.data[row][c].left;
            let right = self.data[row][c].right;

            if left != 0 {
                available[left as usize] = false;
            }

            if right != 0 {
                available[right as usize] = false;
            }
        }

        for r in 0..12 {
            for c in 0..6 {
                let left = self.data[r][c].left;
                let right = self.data[r][c].right;

                if left != 0 && right != 0 {
                    used_pairs.insert((left, right));
                }
            }
        }

        for left_value in 1..=12 {
            if available[left_value as usize] {
                available[left_value as usize] = false;

                // keep copy of available
                let original_available = available;

                for right_value in 1..=12 {
                    if available[right_value as usize] {
                        available[right_value as usize] = false;

                        let pair = (left_value.min(right_value), left_value.max(right_value));

                        if used_pairs.contains(&pair) {
                            continue;
                        }

                        self.data[row][col] = Match::new(left_value, right_value);

                        if self.solve(row, col + 1) {
                            return true;
                        }

                        self.data[row][col] = Match::new(0, 0);
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
