use image::{EncodableLayout, ImageBuffer, ImageEncoder, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use imageproc::drawing::text_size;

use rusttype::{Font, Scale};
use std::borrow::BorrowMut;
use std::collections::HashMap;

const LIGHT: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255u8]);
const DARK: Rgba<u8> = Rgba([255, 0, 0, 255]);

const FONT: &[u8] = include_bytes!("resources/fonts/SegoeUIBold.ttf");
const PAWN_IMAGE: [&[u8]; 2] = [
    include_bytes!("resources/pieces/BP.png"),
    include_bytes!("resources/pieces/WP.png"),
];

const ROOK_IMAGE: [&[u8]; 2] = [
    include_bytes!("resources/pieces/BR.png"),
    include_bytes!("resources/pieces/WR.png"),
];

const KNIGHT_IMAGE: [&[u8]; 2] = [
    include_bytes!("resources/pieces/BN.png"),
    include_bytes!("resources/pieces/WN.png"),
];

const BISHOP_IMAGE: [&[u8]; 2] = [
    include_bytes!("resources/pieces/BB.png"),
    include_bytes!("resources/pieces/WB.png"),
];

const QUEEN_IMAGE: [&[u8]; 2] = [
    include_bytes!("resources/pieces/BQ.png"),
    include_bytes!("resources/pieces/WQ.png"),
];

const KING_IMAGE: [&[u8]; 2] = [
    include_bytes!("resources/pieces/BK.png"),
    include_bytes!("resources/pieces/WK.png"),
];

enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

struct Piece {
    piece_type: PieceType,
    color: bool,
}

pub struct Board {
    board: ImageBuffer<Rgba<u8>, Vec<u8>>,
    tile_size: u32,
    colors: Color,
    pieces: HashMap<(u32, u32), Piece>,
    last_move: Option<((u32, u32), (u32, u32))>,
}

struct Color {
    dark: Rgba<u8>,
    light: Rgba<u8>,
}

impl Piece {
    fn image(&self) -> RgbaImage {
        let bytes = match self.piece_type {
            PieceType::Pawn => PAWN_IMAGE[self.color as usize],
            PieceType::Rook => ROOK_IMAGE[self.color as usize],
            PieceType::Knight => KNIGHT_IMAGE[self.color as usize],
            PieceType::Bishop => BISHOP_IMAGE[self.color as usize],
            PieceType::Queen => QUEEN_IMAGE[self.color as usize],
            PieceType::King => KING_IMAGE[self.color as usize],
        };
        let image = image::load_from_memory(bytes);
        image.unwrap().into_rgba8()
    }
}

pub trait Setup {
    fn normal_board(light: Rgba<u8>, dark: Rgba<u8>, tile_size: u32) -> Board;
    fn empty_board(light: Rgba<u8>, dark: Rgba<u8>, tile_size: u32) -> Board;
}

pub trait Draw {
    fn text_decoration(&mut self, white: bool);
    fn draw_pieces(&mut self, white: bool);
    fn last_move(&mut self, white: bool);
    fn draw_and_render(&mut self, white: bool) -> Vec<u8>;
}

pub trait Encode {
    fn encode_png(&self) -> Vec<u8>;
}

impl Setup for Board {
    fn empty_board(light: Rgba<u8>, dark: Rgba<u8>, tile_size: u32) -> Self {
        let mut board = RgbaImage::new(8, 8);
        for (x, y, pixel) in board.enumerate_pixels_mut() {
            if (x + y) % 2 == 0 {
                *pixel = light;
            } else {
                *pixel = dark;
            }
        }
        let board = image::imageops::resize(
            &board,
            tile_size * 8,
            tile_size * 8,
            image::imageops::FilterType::Nearest,
        );
        let pieces = HashMap::new();
        let colors = Color { dark, light };
        Board {
            board,
            colors,
            tile_size,
            pieces,
            last_move: None,
        }
    }

    fn normal_board(light: Rgba<u8>, dark: Rgba<u8>, tile_size: u32) -> Self {
        let mut board = Self::empty_board(light, dark, tile_size);
        const ROOKS: [(u32, u32); 4] = [(0, 0), (7, 0), (0, 7), (7, 7)];
        const KNIGHTS: [(u32, u32); 4] = [(1, 0), (6, 0), (1, 7), (6, 7)];
        const BISHOPS: [(u32, u32); 4] = [(2, 0), (5, 0), (2, 7), (5, 7)];
        const QUEENS: [(u32, u32); 2] = [(3, 0), (3, 7)];
        const KINGS: [(u32, u32); 2] = [(4, 0), (4, 7)];
        for i in 0..8 {
            board.pieces.insert(
                (i, 1),
                Piece {
                    piece_type: PieceType::Pawn,
                    color: false,
                },
            );
            board.pieces.insert(
                (i, 6),
                Piece {
                    piece_type: PieceType::Pawn,
                    color: true,
                },
            );
        }
        for i in ROOKS.iter() {
            board.pieces.insert(
                *i,
                Piece {
                    piece_type: PieceType::Rook,
                    color: i.1 != 0,
                },
            );
        }
        for i in KNIGHTS.iter() {
            board.pieces.insert(
                *i,
                Piece {
                    piece_type: PieceType::Knight,
                    color: i.1 != 0,
                },
            );
        }
        for i in BISHOPS.iter() {
            board.pieces.insert(
                *i,
                Piece {
                    piece_type: PieceType::Bishop,
                    color: i.1 != 0,
                },
            );
        }
        for i in QUEENS.iter() {
            board.pieces.insert(
                *i,
                Piece {
                    piece_type: PieceType::Queen,
                    color: i.1 != 0,
                },
            );
        }
        for i in KINGS.iter() {
            board.pieces.insert(
                *i,
                Piece {
                    piece_type: PieceType::King,
                    color: i.1 != 0,
                },
            );
        }
        board
    }
}

impl Draw for Board {
    fn text_decoration(&mut self, white: bool) {
        let font = Font::try_from_bytes(FONT).unwrap();
        let font_size = self.tile_size as f32 * 0.3;
        let scale = Scale::uniform(font_size);
        let padding = (font_size / 5.0) as i32;
        let v_metrics = font.v_metrics(scale);
        let counter_bot = v_metrics.ascent as i32;
        let counter_top = v_metrics.descent as i32;
        for (i, c) in (b'a'..=b'h').enumerate() {
            let char = String::from(c as char);
            let char_size = text_size(scale, &font, char.as_str());
            let x = (i as i32 + 1) * self.tile_size as i32 - char_size.0 as i32 - padding;
            let y = (self.tile_size as i32 * 8) - counter_bot as i32 - padding;
            let color = if i % 2 == 0 {
                self.colors.light
            } else {
                self.colors.dark
            };
            draw_text_mut(&mut self.board, color, x, y, scale, &font, &char);
        }

        for (i, c) in (b'1'..=b'8').enumerate() {
            let i = if white { 7 - i } else { i };
            /*             println!("{}", i); */
            let x = padding;
            let y = (i as i32) * self.tile_size as i32 + counter_top + padding;
            let color = if (i + 1) % 2 == 0 {
                self.colors.light
            } else {
                self.colors.dark
            };
            draw_text_mut(
                &mut self.board,
                color,
                x,
                y,
                scale,
                &font,
                (c as char).to_string().as_str(),
            );
        }
    }

    fn draw_pieces(&mut self, white: bool) {
        for ((x, y), piece) in self.pieces.iter() {
            let x = if white { *x } else { 7 - x };
            let y = if white { *y } else { 7 - y };
            let x = x * self.tile_size;
            let y = y * self.tile_size;
            let mut image = piece.image();
            if !(image.width() == self.tile_size && image.height() == self.tile_size) {
                image = image::imageops::resize(
                    &image,
                    self.tile_size,
                    self.tile_size,
                    image::imageops::FilterType::Nearest,
                );
            }

            image::imageops::overlay(&mut self.board, &image, x.into(), y.into());
        }
    }

    fn last_move(&mut self, white: bool) {
        if let Some((from, to)) = self.last_move {
            let from = if white {
                from
            } else {
                (7 - from.0, 7 - from.1)
            };
            let dark = self.colors.dark.0;
            let light = self.colors.light.0;
            let dark_merge = Rgba([dark[0], dark[1], dark[2], 100]);
            let light_merge = Rgba([light[0], light[1], light[2], 100]);

            let to = if white { to } else { (7 - to.0, 7 - to.1) };
            let from_pixels = (from.0 * self.tile_size, from.1 * self.tile_size);
            let to_pixels = (to.0 * self.tile_size, to.1 * self.tile_size);
            let mut image_from = image::RgbaImage::new(self.tile_size, self.tile_size);
            let color_from = if (from.0 + from.1) % 2 == 0 {
                println!("light");
                dark_merge
            } else {
                light_merge
            };
            for pixel in image_from.pixels_mut() {
                *pixel = color_from;
            }
            let mut image_to = image::RgbaImage::new(self.tile_size, self.tile_size);
            let color_to = if (to.0 + to.1) % 2 == 0 {
                println!("light");
                dark_merge
            } else {
                light_merge
            };
            for pixel in image_to.pixels_mut() {
                *pixel = color_to;
            }
            image::imageops::overlay(
                &mut self.board,
                &image_from,
                from_pixels.0.into(),
                from_pixels.1.into(),
            );
            image::imageops::overlay(
                &mut self.board,
                &image_to,
                to_pixels.0.into(),
                to_pixels.1.into(),
            );
        }
    }

    fn draw_and_render(&mut self, white: bool) -> Vec<u8> {
        let img = self.board.clone();
        self.text_decoration(white);
        self.draw_pieces(white);
        self.last_move(white);
        let png = self.encode_png();
        self.board = img;
        png
    }
}

impl Encode for Board {
    fn encode_png(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let bytes = self.board.as_bytes().to_vec();
        image::codecs::png::PngEncoder::new(&mut buffer)
            .write_image(
                &bytes,
                self.tile_size * 8,
                self.tile_size * 8,
                image::ColorType::Rgba8,
            )
            .unwrap();
        buffer
    }
}
