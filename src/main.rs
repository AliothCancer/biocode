use biocode::{
    cells::{Cell, Coor},
    dna::Dna,
};
use macroquad::prelude::*;

#[macroquad::main("Mutazioni Complesse (Indel)")]
async fn main() {
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;
    let center = Coor::new(center_x, center_y);
    let dna = Dna::new("TTTTATTTT");
    let cell_pent = Cell::new(Dna::new("TTTTT"), center);
    let cell_comp = Cell::new(dna, center + Coor::new(200.0, 0.0));
    let fusion = (&cell_pent + &cell_comp).with_center(center + Coor::new(0.0, 200.0));

    let cells = [cell_pent.clone(), cell_comp.clone(), fusion];

    loop {
        clear_background(BLACK);
        for cell in cells.iter() {
            cell.draw();
        }
        next_frame().await
    }
}
