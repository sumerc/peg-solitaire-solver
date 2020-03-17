#[derive(Debug)]
struct Board {
    board: u64,
}

#[derive(Debug)]
struct EnglishBoard {
    neighbour_matrix: [[u8; 9]; 9],
}

impl Default for EnglishBoard {
    #[inline]
    fn default() -> EnglishBoard {
        EnglishBoard {
            neighbour_matrix: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 2, 3, 0, 0, 0],
                [0, 0, 0, 4, 5, 6, 0, 0, 0],
                [0, 7, 8, 9,10,11,12,13, 0],
                [0,14,15,16,17,18,19,20, 0],
                [0,21,22,23,24,25,26,27, 0],
                [0, 0, 0,28,29,30, 0, 0, 0],
                [0, 0, 0,31,32,33, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        }
    }
}

// impl std::ops::Index<[usize; 2]> for Matrix {
//     type Output = f32;

//     fn index(&self, idx: [usize; 2]) -> &f32 {
//         // or as appropriate for row- or column-major data
//         &self.data[idx[0] * self.cols + idx[1]]
//     }
// }

impl Board {
    fn available_moves(x: usize, y: usize) -> u8 {
        return 0;
    }
}

fn main() {
    //let board = Board {board: 1};
    //let english_board = EnglishBoard { ..Default::default() };
    let english_board: EnglishBoard = Default::default();

    println!("board: {:?}", english_board);
}
