use macroquad::prelude;
use std::collections::HashMap;
use once_cell::sync::Lazy;




mod piece;
mod engine;
mod board;
mod render;

use crate::piece::*;
use crate::engine::*;
use crate::board::*;
use crate::render::*;



fn get_config() -> prelude::Conf {
    prelude::Conf { window_title: "chess".to_string(), fullscreen: true, window_resizable: true, ..Default::default() }
}

#[macroquad::main(get_config)]
async fn main() {
    let piece_images: HashMap<u8, prelude::Image> = load_pieces().await;

    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w".to_string());

    loop {
        if prelude::is_key_pressed(prelude::KeyCode::Escape) {
            break;
        }

        render(&mut board, &piece_images);
        prelude::next_frame().await;
    }
}


pub async fn load_pieces() -> HashMap<u8, prelude::Image> {
    let mut  pieces: HashMap<u8, prelude::Image> = HashMap::new();
    
    pieces.insert(WHITE | PAWN, prelude::load_image("assets/wp.png").await.unwrap());
    pieces.insert(WHITE | BISHOP, prelude::load_image("assets/wb.png").await.unwrap());
    pieces.insert(WHITE | ROOK, prelude::load_image("assets/wr.png").await.unwrap());
    pieces.insert(WHITE | QUEEN, prelude::load_image("assets/wq.png").await.unwrap());
    pieces.insert(WHITE | KING, prelude::load_image("assets/wk.png").await.unwrap());
    pieces.insert(WHITE | NIGHT, prelude::load_image("assets/wn.png").await.unwrap());

    pieces.insert(BLACK | PAWN, prelude::load_image("assets/bp.png").await.unwrap());
    pieces.insert(BLACK | BISHOP, prelude::load_image("assets/bb.png").await.unwrap());
    pieces.insert(BLACK | ROOK, prelude::load_image("assets/br.png").await.unwrap());
    pieces.insert(BLACK | QUEEN, prelude::load_image("assets/bq.png").await.unwrap());
    pieces.insert(BLACK | KING, prelude::load_image("assets/bk.png").await.unwrap());
    pieces.insert(BLACK | NIGHT, prelude::load_image("assets/bn.png").await.unwrap());

    pieces
}






