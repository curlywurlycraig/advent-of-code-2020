use std::fs::File;
use std::io::prelude::*;

struct Forest {
    rows: Vec<String>
}

impl From<String> for Forest {
    fn from(input: String) -> Self {
        let rows = input.split("\n").map(|s| String::from(s)).collect();

        Forest {
            rows
        }
    }
}

impl<'a> Forest {
    fn zoom(&'a self, xvel: usize, yvel: usize) -> ForestIter<'a> {
        ForestIter {
            forest: self,
            column: 0,
            row: 0,
            xvel,
            yvel
        }
    }
}

struct ForestIter<'a> {
    row: usize,
    column: usize,
    xvel: usize,
    yvel: usize,
    forest: &'a Forest
}

impl<'a> ForestIter<'a> {
    fn total(&mut self) -> usize {
        self.filter(|v| *v).count()
    }
}

impl<'a> Iterator for ForestIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.forest.rows.len() {
            return None
        }

        let row_len = self.forest.rows.get(0).unwrap().len();

        let current_character = self
            .forest.rows
            .get(self.row)
            .unwrap()
            .chars()
            .nth(self.column % row_len)
            .unwrap();

        let result = match current_character {
            '#' => true,
            '.' => false,
            _ => panic!("Non tree or ground. Are you in space? {}", current_character)
        };

        self.row += self.yvel;
        self.column += self.xvel;

        Some(result)
    }
}

fn main() {
    let mut file = File::open("src/index.txt").unwrap();
    let mut forest_string = String::new();
    file.read_to_string(&mut forest_string).unwrap();

    let forest = Forest::from(forest_string);
    let tree_count = forest
        .zoom(1, 3)
        .total();

    println!("Encountered {} trees", tree_count);

    let mult_total: usize = vec![
        forest.zoom(1, 1).total(),
        forest.zoom(3, 1).total(),
        forest.zoom(5, 1).total(),
        forest.zoom(7, 1).total(),
        forest.zoom(1, 2).total()
    ].iter().product();

    println!("Total product: {}", mult_total);
}
