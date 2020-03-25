use std::collections::HashSet;

#[inline]
fn get_val(board: u64, index: u8) -> bool {
    if index == 0 {
        return false;
    }
    return board & (1 << (index - 1)) > 0;
}

#[inline]
fn set_val(board: &mut u64, index: u8, val: u8) {
    assert_ne!(index, 0);

    let mask = 1 << (index-1) as u64;

    if val > 0 {
        *board = *board | mask;
    } else {
        *board = *board & !(mask)
    }
}

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

    // fn can_move_from(&self, board: u64, y: u8, x: u8) -> Option<u64> {

    // }

    fn myprint(&self, board: &u64)
    {
        let nmatrix = self.neighbour_matrix();

        println!("{}", "--".repeat(nmatrix.len()));
        for (_y,row) in nmatrix.iter().enumerate() {
            for (_x, val) in row.iter().enumerate() {
                // For a move to happen, following conditions required:
                //   - an empty slot is available
                //   - there is a non-empty neighbour slot
                //   - there is a non-empty slot which we can jump over the
                //     non-empty neighbour from four directions(N,S,E,W)
                if *val > 0 {
                    let bval = get_val(*board, *val);
                    print!("{} ", bval as u8);
                } else {
                    print!("  ");
                }
            }
            print!("\n");
        }
        println!("{}", "--".repeat(nmatrix.len()));
    }

    fn available_moves(&self, board: u64) -> HashSet<u64> {
        let mut result = HashSet::new();
        self.myprint(&board);
        
        let nmatrix = self.neighbour_matrix();
        //println!("get_neighbour_matrix: {:?}", self.neighbour_matrix());
        for (y,row) in nmatrix.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                // For a move to happen, following conditions required:
                //   - an empty slot is available
                //   - there is a non-empty neighbour slot
                //   - there is a non-empty slot which we can jump over the
                //     non-empty neighbour from four directions(N,S,E,W)
                if *val > 0 {
                    let bval = get_val(board, *val);
                    if !bval {
                        // top?
                        if y >= 2 {
                            if get_val(board, nmatrix[y-1][x]) && get_val(board, nmatrix[y-2][x]) {
                                let mut board_copy = board;
                                set_val(&mut board_copy, nmatrix[y-2][x], 0);
                                set_val(&mut board_copy, nmatrix[y-1][x], 0);
                                set_val(&mut board_copy, nmatrix[y][x], 1);

                                result.insert(board_copy);
                            }
                        }
                        // left?
                        if x >= 2 {
                            if get_val(board, nmatrix[y][x-1]) && get_val(board, nmatrix[y][x-2]) {
                                let mut board_copy = board;
                                set_val(&mut board_copy, nmatrix[y][x-2], 0);
                                set_val(&mut board_copy, nmatrix[y][x-1], 0);
                                set_val(&mut board_copy, nmatrix[y][x], 1);

                                result.insert(board_copy);
                            }
                        }
                        // bottom
                        if y < nmatrix.len()-2 {
                            if get_val(board, nmatrix[y+1][x]) && get_val(board, nmatrix[y+2][x]) {
                                let mut board_copy = board;
                                set_val(&mut board_copy, nmatrix[y+2][x], 0);
                                set_val(&mut board_copy, nmatrix[y+1][x], 0);
                                set_val(&mut board_copy, nmatrix[y][x], 1);

                                result.insert(board_copy);
                            }
                        }
                        // right
                        if x < row.len()-2 {
                            if get_val(board, nmatrix[y][x+1]) && get_val(board, nmatrix[y][x+2]) {
                                let mut board_copy = board;
                                set_val(&mut board_copy, nmatrix[y][x+2], 0);
                                set_val(&mut board_copy, nmatrix[y][x+1], 0);
                                set_val(&mut board_copy, nmatrix[y][x], 1);

                                result.insert(board_copy);
                            }
                        }
                    }
                }
            }
        }

        // iterate over the board bitmap, find empty slots and
        return result;
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
    let moves = english_board.available_moves(0xFFFF_FFFF_FFFE_FFFF);

    for mv in &moves {
        english_board.myprint(&mv);
    }

    let x = moves.iter().next().unwrap();
    println!("{:x}", x);
    english_board.myprint(x);

    let moves = english_board.available_moves(*x);
    for mv in &moves {
        english_board.myprint(&mv);
    }

    //println!("bb={}", 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111 as u64);

    // println!(
    //     "board: {:?} {:p} {:p} {}",
    //     english_board.neighbour_matrix(),
    //     english_board.neighbour_matrix(),
    //     english_board.neighbour_matrix(),
    //     english_board.board_len(),
    // );
}
