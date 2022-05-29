use std::fmt;

#[derive(Debug, Clone)]
pub struct DsuError {
    msg: String,
}
impl DsuError {
    pub fn new(msg: &str) -> Self {
        DsuError { msg: msg.to_string() }
    }
}
impl fmt::Display for DsuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

pub struct Dsu {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Dsu {
            parents: (0..n).collect(),
            ranks: vec![1; n],
        }
    }

    pub fn lookup(&mut self, key: usize) -> Result<usize, DsuError> {
        if key >= self.parents.len() {
            return Err(DsuError::new("Unable create item. Exceeded set size"));
        }
        if key == self.parents[key] {
            return Ok(key);
        }
        self.parents[key] = self.lookup(self.parents[key])?;
        return Ok(self.parents[key]);
    }

    pub fn union(&mut self, first: usize, second: usize) -> Result<(), DsuError> {
        let first = self.lookup(first)?;
        let second = self.lookup(second)?;

        if first != second {
            if self.ranks[first] < self.ranks[second] {
                self.ranks.swap(first, second);
            }
            self.parents[second] = first;
            if self.ranks[first] == self.ranks[second] {
                self.ranks[first] += 1;
            }
        }
        Ok(())
    }
}
