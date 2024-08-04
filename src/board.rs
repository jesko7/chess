use macroquad::miniquad::BlendFactor;

use crate::piece::*;

pub fn square_to_index(square: (u8, u8)) -> usize {
    (square.1 * 8 + square.0) as usize
}

pub struct Board {
    pub pieces: Vec<Option<Piece>>,
    pub to_move: u8,
}

impl Board {
    pub fn execute_move(&mut self, start_square: (u8, u8), end_square: (u8, u8)) {
        let start_index = square_to_index(start_square);
        let end_index = square_to_index(end_square);

        let mut piece = self.pieces[start_index].clone().unwrap();
        self.pieces[start_index] = None;
        piece.has_moved = true;
        piece.square = end_square;
        self.pieces[end_index] = Some(piece);
        
    }

    pub fn from_fen(fen: String) -> Board {
        let mut pieces: Vec<Option<Piece>> = vec![];

        let mut x: u8 = 0;
        let mut y: u8 = 0;

        let fen_to_move = fen.split(" ").collect::<Vec<&str>>();
        let fen = fen_to_move[0];
        let to_move = match fen_to_move[1] {
            "w" => WHITE,
            "b" => BLACK,

            other => panic!()
        };


        for fen_char in fen.chars() {
            if fen_char == '/' {
                y += 1;
                x = 0;
                continue;
            }
            if let Ok(num) = fen_char.to_string().parse::<u8>() {
                x += num;
                for _ in 0..num {
                    pieces.push(None);
                }
            }
            else {
                let piece_type = match fen_char {
                    'p' => BLACK | PAWN,
                    'b' => BLACK | BISHOP,
                    'n' => BLACK | NIGHT,
                    'r' => BLACK | ROOK,
                    'q' => BLACK | QUEEN,
                    'k' => BLACK | KING,

                    'P' => WHITE | PAWN,
                    'B' => WHITE | BISHOP,
                    'N' => WHITE | NIGHT,
                    'R' => WHITE | ROOK,
                    'Q' => WHITE | QUEEN,
                    'K' => WHITE | KING,

                    other => panic!()
                };

                let square = (x, y);

                let piece = Piece {
                    square,
                    piece_type,
                    has_moved: false
                };

                pieces.push(Some(piece));

                x += 1;
            }
        }

        Board { pieces, to_move }
    }
}