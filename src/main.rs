use macroquad::prelude::*;
use std::f32::consts::PI;

// 1. Definiamo lo stato del nostro cursore
struct Cursor {
    x: f32,
    y: f32,
    angle: f32,
    radius: f32,
}

// 2 & 3. La funzione che traduce il DNA in istruzioni visive
fn draw_organism(dna: &str, start_x: f32, start_y: f32) {
    let mut cursor = Cursor {
        x: start_x,
        y: start_y,
        angle: -PI / 2.0, // Punta verso l'alto dello schermo (-90 gradi)
        radius: 15.0,
    };

    for nucleotide in dna.chars() {
        match nucleotide {
            'A' => {
                // Disegna una "cellula" e sposta il cursore in avanti
                draw_circle(
                    cursor.x,
                    cursor.y,
                    cursor.radius,
                    Color::new(0.2, 0.8, 0.2, 1.0),
                );
                cursor.x += cursor.angle.cos() * (cursor.radius * 2.2);
                cursor.y += cursor.angle.sin() * (cursor.radius * 2.2);
            }
            'C' => {
                // Modifica la rotta: ruota a destra di 45 gradi
                cursor.angle += PI / 4.0;
            }
            'G' => {
                // Modifica la rotta: ruota a sinistra di 45 gradi
                cursor.angle -= PI / 4.0;
            }
            'T' => {
                // Disegna un legame strutturale e rimpicciolisce i nodi futuri
                let next_x = cursor.x + cursor.angle.cos() * 30.0;
                let next_y = cursor.y + cursor.angle.sin() * 30.0;
                draw_line(cursor.x, cursor.y, next_x, next_y, 4.0, DARKGRAY);

                cursor.x = next_x;
                cursor.y = next_y;
                cursor.radius *= 0.85; // Le propaggini diventano più piccole
            }
            _ => {} // Ignora eventuali mutazioni invalide o caratteri di scarto
        }
    }
}

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

        // Genera l'organismo partendo dal centro dello schermo
        draw_organism(&organism_dna, screen_width() / 2.0, screen_height() / 2.0);

        next_frame().await
    }
}
