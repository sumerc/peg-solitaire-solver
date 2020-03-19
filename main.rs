use std::collections::HashSet;

trait Board {
    //#[inline]
    fn neighbour_matrix(&self) -> &'static [[u8; 9]; 9] {
        return &[
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
    }

    fn board_len(&self) -> usize {
        return 0;
    }

    fn available_moves(&self, board: u64) -> HashSet<u64> {
        let mut result = HashSet::new();
        //result.insert(1);
        
        //println!("get_neighbour_matrix: {:?}", self.neighbour_matrix());
        for row in self.neighbour_matrix().iter() {
            for val in row.iter() {
                if (*val > 0) {
                    println!("cell={:?}", val)
                }
            }
        }

        // iterate over the board bitmap, find empty slots and

        result
    }
}

#[derive(Debug)]
struct EnglishBoard {}

impl Board for EnglishBoard {
    #[inline]
    fn neighbour_matrix(&self) -> &'static [[u8; 9]; 9] {
        return &[
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 2, 3, 0, 0, 0],
            [0, 0, 0, 4, 5, 6, 0, 0, 0],
            [0, 7, 8, 9, 10, 11, 12, 13, 0],
            [0, 14, 15, 16, 17, 18, 19, 20, 0],
            [0, 21, 22, 23, 24, 25, 26, 27, 0],
            [0, 0, 0, 28, 29, 30, 0, 0, 0],
            [0, 0, 0, 31, 32, 33, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
    }

    #[inline]
    fn board_len(&self) -> usize {
        return 33;
    }
    
}

// impl std::ops::Index<[usize; 2]> for Matrix {
//     type Output = f32;

//     fn index(&self, idx: [usize; 2]) -> &f32 {
//         // or as appropriate for row- or column-major data
//         &self.data[idx[0] * self.cols + idx[1]]
//     }
// }

fn main() {
    //let board = Board {board: 1};
    let english_board = EnglishBoard {};
    let a = english_board.available_moves(1);

    println!(
        "board: {:?} {:p} {:p} {}",
        english_board.neighbour_matrix(),
        english_board.neighbour_matrix(),
        english_board.neighbour_matrix(),
        english_board.board_len(),
    );

    //println!("board: {:p}", &a);
}
