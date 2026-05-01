use macroquad::prelude::*;
// 4. Il loop di gioco principale
#[macroquad::main("Procedural Bio Game")]
async fn main() {
    // Prova a cambiare questa stringa per vedere forme radicalmente diverse
    let mut organism_dna = "AACAAGATAGTAACAA".to_string();
    for i in 0..20 {
        organism_dna = organism_dna.replace("T", "ACT");
    }
    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Sfondo scuro

        next_frame().await
    }
}
