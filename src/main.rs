use biocode::{
    cells::{Cell, Coor},
    dna::Dna,
};
use macroquad::prelude::*;

#[macroquad::main("Mutazioni Complesse (Indel)")]
async fn main() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    let cell = Cell::new(Dna::new("ACGATC"), Coor::new(center_x, center_y));
    loop {
        clear_background(BLACK);
        cell.draw();
        next_frame().await
    }
}
