use std::collections::HashMap;

use macroquad::prelude;

use crate::board::*;
use crate::piece::*;
use crate::render::*;

mod board;
mod engine;
mod piece;
mod render;

fn get_config() -> prelude::Conf {
    prelude::Conf {
        window_title: "chess".to_string(),
        fullscreen: true,
        window_resizable: true,
        ..Default::default()
    }
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
    let mut pieces: HashMap<u8, prelude::Image> = HashMap::new();

    pieces.insert(
        WHITE | PAWN,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\wp.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        WHITE | BISHOP,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\wb.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        WHITE | ROOK,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\wr.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        WHITE | QUEEN,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\wq.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        WHITE | KING,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\wk.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        WHITE | NIGHT,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\wn.png")
            .await
            .unwrap(),
    );

    pieces.insert(
        BLACK | PAWN,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\bp.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        BLACK | BISHOP,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\bb.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        BLACK | ROOK,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\br.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        BLACK | QUEEN,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\bq.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        BLACK | KING,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\bk.png")
            .await
            .unwrap(),
    );
    pieces.insert(
        BLACK | NIGHT,
        prelude::load_image(r"C:\Users\jjput\RustroverProjects\chess\assets\bn.png")
            .await
            .unwrap(),
    );

    pieces
}
