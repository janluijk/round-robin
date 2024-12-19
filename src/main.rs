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
    data: [[Match; 6]; 11],
}

impl Matchup {
    fn new(default_value: Match) -> Self {
        Self {
            data: [[default_value; 6]; 11],
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

    fn find_solution(&mut self) -> bool {
        self.solve()
    }

    fn solve(&mut self) -> bool {
        for left in 1..=12 {
            for right in 1..=12 {
                if left == right {
                    continue;
                }
                self.check_pair(left, right);
            }
        }

        true
    }

    fn check_pair(&mut self, left: usize, right: usize) -> bool {
        for row in 0..11 {
            for col in 0..6 {
                let entry = self.data[row][col];
                if entry.left != 0 || entry.right != 0 {
                    continue;
                }

                if self.check_position(left, right, row, col) {
                    return true;
                }
            }
        }

        false
    }

    fn check_position(&mut self, left: usize, right: usize, row: usize, col: usize) -> bool {
        let mut available: u32 = 0b1111111111110;

        let range = if row <= 6 { 0..6 } else { 6..11 };
        for r in range {
            let entry = self.data[r][col];
            if entry.left != 0 || entry.right != 0 {
                available &= !(1 << entry.left);
                available &= !(1 << entry.right);
            }
        }

        for c in 0..6 {
            let entry = self.data[row][c];
            if entry.left != 0 || entry.right != 0 {
                available &= !(1 << entry.left);
                available &= !(1 << entry.right);
            }
        }

        if (available & (1 << left) != 0) && (available & (1 << right) != 0) {
            self.data[row][col] = Match::new(left as i32, right as i32);
            println!("{}, {} placed at {} {}", left, right, row, col);
            return true;
        }
        false
    }
}

fn main() {
    let default_value = Match::new(0, 0);
    let mut matchup = Matchup::new(default_value);

    if matchup.find_solution() {
        println!("Solution found");

        matchup.print();
    } else {
        println!("No solution found.");
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
