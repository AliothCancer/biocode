use macroquad::math::Vec2;

use crate::{coordinates::Coor, dna::Dna};

// Livello 1: L'Organismo
pub struct Cell {
    pub center: Vec2,
    pub nucleus: Nucleus,
    pub membrane: Membrane,
    pub dna: Dna,
}

pub struct Nucleus {
    // Il nucleo potrebbe contenere le riserve di energia o la logica di riproduzione
}

pub struct Membrane {
    pub organs: Vec<Organ>,
}

// Livello 2: L'Organo (Il "Building Block" funzionale)
pub struct Organ {
    pub kind: OrganKind,
    pub segments: Vec<Segment>,
    pub local_offset: Coor, // Dove si trova l'organo rispetto al centro della cellula
}

pub enum OrganKind {
    Mouth,
    Sensor,
    Flagellum,
    Armor,
}

// Livello 3: Il Segmento (L'unità atomica)
pub struct Segment {
    pub kind: SegmentKind,
    pub start: Coor, // Relative al centro dell'organo
    pub end: Coor,
}

pub enum SegmentKind {
    Structural,
    Absorber,
    Core,
    Thruster,
}

impl Cell {
    pub fn new(center: Coor, dna: Dna) -> Self {
        todo!()
    }
}
