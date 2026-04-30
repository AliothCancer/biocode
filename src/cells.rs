use crate::dna::Dna;

pub struct Cell {
    nucleus: Nucleus,
    membrane: Membrane,
    dna: Dna,
}
pub struct Nucleus(Vec<Segment>);
pub struct Membrane(Vec<Segment>);

impl Cell {
    pub fn new(dna: Dna) -> Self {
        Self {
            nucleus: Nucleus(vec![]),
            membrane: Membrane(vec![]),
            dna,
        }
    }
}

pub struct Segment {
    start: (f32, f32),
    end: (f32, f32),
    kind: SegmentKind,
}

pub enum SegmentKind {
    Structural,
    Core,
    Absorber,
    Sensor,
}
