use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::dna::{Dna, GENES_NUM};

// Rimosso ANGLE_STEP globale!

#[derive(Clone, Copy)]
pub struct Coor {
    pub x: f32,
    pub y: f32,
}
impl Coor {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Cell {
    pub dna: Dna,
    pub center: Coor,
    pub expressed: Vec<Coor>,
}

impl Cell {
    pub fn new(dna: Dna, center: Coor) -> Self {
        let mut cell = Self {
            dna,
            center,
            expressed: Vec::with_capacity(GENES_NUM),
        };
        cell.express_phenotype();
        cell
    }

    pub fn express_phenotype(&mut self) {
        self.expressed.clear();

        // 1. Contiamo istantaneamente i bit a 1 (geni attivi)
        let active_genes = self.dna.activity_mask.count_ones();

        // Se non ci sono almeno 3 geni attivi, non si può formare un poligono.
        // Usciamo subito senza calcolare nulla.
        if active_genes < 3 {
            return;
        }

        // 2. L'angolo dinamico diviso per il numero reale di vertici
        let angle_step = 2.0 * PI / active_genes as f32;

        // 3. Teniamo traccia dell'angolo effettivo su cui stiamo disegnando
        let mut current_angle = 0.0_f32;

        for i in 0..GENES_NUM {
            let is_active = (self.dna.activity_mask >> i) & 1 == 1;

            if is_active {
                let radius = match self.dna.sequence[i] {
                    'A' => 20.0,
                    'C' => 40.0,
                    'G' => 60.0,
                    'T' => 80.0,
                    _ => 30.0,
                };

                self.expressed.push(Coor {
                    x: self.center.x + current_angle.cos() * radius,
                    y: self.center.y + current_angle.sin() * radius,
                });

                // Avanziamo con l'angolo SOLO dopo aver espresso un gene
                current_angle += angle_step;
            }
        }
    }

    pub fn draw(&self) {
        let num_points = self.expressed.len();

        if num_points < 3 {
            draw_circle(self.center.x, self.center.y, 5.0, GRAY);
            return;
        }

        for i in 0..num_points {
            let p1 = self.expressed[i];
            let p2 = self.expressed[(i + 1) % num_points];

            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, GREEN);
        }

        draw_circle(self.center.x, self.center.y, 4.0, RED);
    }
}
