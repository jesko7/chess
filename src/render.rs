use crate::board::*;
use crate::piece::*;

use macroquad::prelude;
use std::collections::HashMap;

pub static mut selected_pice: Option<Piece> = None; 

pub fn square_to_coords(square: (u8, u8)) -> (f32, f32) {
    (square.0 as f32 * PIECE_DIMENSION + 480., square.1 as f32 * PIECE_DIMENSION + 60.)
}
pub fn coords_to_square(coords: (f32, f32)) -> (u8, u8) {
    (((coords.0 - 480.) / PIECE_DIMENSION) as u8, ((coords.1 - 60.) / PIECE_DIMENSION) as u8)
}

pub fn render(board: &mut Board, piece_images: &HashMap<u8, prelude::Image> ) {
    prelude::clear_background(prelude::DARKGRAY);

    if prelude::is_mouse_button_pressed(prelude::MouseButton::Left) {
        let square = coords_to_square(prelude::mouse_position());
        let index = square_to_index(square);
        unsafe {
            let coords = square_to_coords(square);
            selected_pice = board.pieces[index].clone();
        }
    }
    if prelude::is_mouse_button_released(prelude::MouseButton::Left) {
        unsafe {
            if selected_pice.is_some() {
                let end_square = coords_to_square(prelude::mouse_position());
                let start_square = selected_pice.clone().unwrap().square;

                if start_square != end_square {
                    board.execute_move(start_square, end_square);
                }
            }  
        
            selected_pice = None;
        }
    }

    for x in 0..8 {
        for y in 0..8 {
            let coords = square_to_coords((x, y));

            if (x + y) % 2 != 0 {
                prelude::draw_rectangle(coords.0, coords.1, PIECE_DIMENSION, PIECE_DIMENSION, prelude::Color::from_hex(0xa87965))
            }
            else {
                prelude::draw_rectangle(coords.0, coords.1, PIECE_DIMENSION, PIECE_DIMENSION, prelude::Color::from_hex(0xf0d8c0))
            }
        }
    }

    unsafe {
        if let Some(piece) = selected_pice.clone() {
            let legal_moves = piece.get_legal_moves(&board);
            
            for legal_move in legal_moves {
                let cords = square_to_coords(legal_move); 
                prelude::draw_rectangle(cords.0, cords.1, PIECE_DIMENSION, PIECE_DIMENSION, prelude::RED);
            }
        }
    }

    for piece in board.pieces.iter() {
        if let Some(inner_piece) = piece {
            unsafe {
                if Some(inner_piece) == selected_pice.as_ref() {
                    let mut coords = prelude::mouse_position();
                    coords.0 -= PIECE_DIMENSION / 2.;
                    coords.1 -= PIECE_DIMENSION / 2.;
                    let image = piece_images.get(&inner_piece.piece_type).unwrap();
                    prelude::draw_texture_ex(&prelude::Texture2D::from_image(image), coords.0, coords.1, prelude::WHITE, prelude::DrawTextureParams { dest_size: Some(prelude::Vec2 { x: PIECE_DIMENSION, y: PIECE_DIMENSION }), ..Default::default()})
                }
                else {
                    let coords = square_to_coords(inner_piece.square);
                    let image = piece_images.get(&inner_piece.piece_type).unwrap();
                    prelude::draw_texture_ex(&prelude::Texture2D::from_image(image), coords.0, coords.1, prelude::WHITE, prelude::DrawTextureParams { dest_size: Some(prelude::Vec2 { x: PIECE_DIMENSION, y: PIECE_DIMENSION }), ..Default::default()})
                }  
            }
        }
    }
}