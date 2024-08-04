pub const PAWN: u8       = 0b00001;
pub const BISHOP: u8     = 0b00010;
pub const NIGHT: u8      = 0b00011;
pub const ROOK: u8       = 0b00100;
pub const QUEEN: u8      = 0b00101;
pub const KING: u8       = 0b00110;

pub const WHITE: u8      = 0b01000;
pub const BLACK: u8      = 0b10000;

pub const PIECE_MASK: u8 = 0b00111;
pub const COLOR_MASK: u8 = 0b11000;

pub const PIECE_DIMENSION: f32 = 120.;


use std::ops::Index;

use crate::board::*;

pub fn get_sliding_moves(start_square: (u8, u8), color: u8, board: &Board, dir_offsets: Vec<(i16, i16)>) -> Vec<(u8, u8)> {
    let mut legal_squares = vec![];

    for dir_offset in dir_offsets {
        let mut pos = (start_square.0 as i16, start_square.1 as i16);
        loop {
            pos.0 += dir_offset.0;
            pos.1 += dir_offset.1;

            if pos.0 > 7 || pos.0 < 0 || pos.1 > 7 || pos.1 < 0 {
                break;
            }

            let index = square_to_index((pos.0 as u8, pos.1 as u8));

            let piece_at_pos = &board.pieces[index];


            if let Some(piece) = piece_at_pos {
                if piece.get_color() == color {
                    break;
                }
                else {
                    legal_squares.push(pos);
                    break;
                }
            }
            
            legal_squares.push(pos);
        }
    }

    println!("legal_squares: {:?}", legal_squares);

    legal_squares.iter().map(|square| (square.0 as u8, square.1 as u8)).collect::<Vec<(u8, u8)>>()
}

pub fn get_rook_moves(start_square: (u8, u8), color: u8, board: &Board) -> Vec<(u8, u8)> {
    let dir_offsets = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    get_sliding_moves(start_square, color, board, dir_offsets)
}

pub fn get_bishop_moves(start_square: (u8, u8), color: u8, board: &Board) -> Vec<(u8, u8)> {
    let dir_offsets = vec![
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
    ];
    get_sliding_moves(start_square, color, board, dir_offsets)
}

pub fn get_queen_moves(start_square: (u8, u8), color: u8, board: &Board) -> Vec<(u8, u8)> {
    let dir_offsets = vec![
        (-1, -1),
        (1, -1),
        (-1, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    get_sliding_moves(start_square, color, board, dir_offsets)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub square: (u8, u8),
    pub piece_type: u8,
    pub has_moved: bool,
}

pub fn get_name(piece_type: u8) -> String {
    let color = match piece_type & COLOR_MASK {
        WHITE => "w",
        BLACK => "b",
        other => panic!()
    };


    let piece = match piece_type & PIECE_MASK  {
        ROOK => "r",
        PAWN => "p",
        NIGHT => "n",
        KING => "k",
        QUEEN => "q",
        BISHOP => "b",
        other => panic!()
    };

    return color.to_string() + piece;
}

impl Piece {
    pub fn get_color(&self) -> u8 {
        self.piece_type & COLOR_MASK
    }
    pub fn get_piece(&self) -> u8 {
        self.piece_type & PIECE_MASK
    }

    pub fn get_legal_moves(&self, board: &Board) -> Vec<(u8, u8)> {
        match self.get_piece() {
            BISHOP => get_bishop_moves(self.square, self.get_color(), board),
            ROOK => get_rook_moves(self.square, self.get_color(), board),
            QUEEN => get_queen_moves(self.square, self.get_color(), board),
            other => vec![]
        }
    }
}
