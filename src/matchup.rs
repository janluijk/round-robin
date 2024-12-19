#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Match {
    pub left: i32,
    pub right: i32,
}

impl Match {
    pub fn new(left: i32, right: i32) -> Self {
        Match { left, right }
    }
}

pub struct Matchup {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<Match>>,
}

impl Matchup {
    pub fn new(size: usize, default_value: Match) -> Self {
        // Calculate rows based on the formula: rows = cols * 2 - 1
        let cols = size;
        let rows = size * 2 - 1;

        // Initialize the data matrix with the default_value
        let data = vec![vec![default_value; cols]; rows];

        Self { rows, cols, data }
    }

    pub fn print(&self) {
        for row in self.data.iter() {
            for entry in row.iter() {
                print!("({}) ({}), ", entry.left, entry.right);
            }
            println!();
        }
        println!("-----------");
    }

    pub fn solve(&self) {}

    pub fn clean(&mut self) {
        for row in &mut self.data {
            for entry in row {
                entry.left = 0;
                entry.right = 0;
            }
        }
    }
}
