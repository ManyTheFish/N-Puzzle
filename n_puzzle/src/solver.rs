pub struct Board {
    pub data: Box<[u32]>,
    pub line_size: usize,
}

// Public
impl Board {
    pub fn new(data: Box<[u32]>, line_size: usize) -> Self {
        Self {
            data: data,
            line_size: line_size
        }
    }
}

// Private
impl Board {
    fn inversions(&self) -> usize {
        let mut inversions = 0;
        let mut i = 0;
        for current in self.data.iter() {
             inversions += self.data.iter().skip(i).map(|x| {
                if *x < *current && *current != 0 && *x != 0 {
                    1
                } else {
                    0
                }
            }).sum::<usize>();
            i += 1;
        }
        inversions
    }
}

pub struct Solver {
    board: Board,
    expected: Board,
}

// Public
impl Solver {
    pub fn new(board: Board, expected: Board) -> Self {
        Self {
            board: board,
            expected: expected
        }
    }
}