use crate::coordinates::Coor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SegmentType {
    Structural,
    Thruster,
    Mouth,
    Sensor,
}

pub fn translate_codon(codon: &[char]) -> SegmentType {
    match codon {
        ['G', _, 'G'] => SegmentType::Thruster,
        ['C', 'C', _] | ['C', _, 'C'] | [_, 'C', 'C'] => SegmentType::Mouth,
        ['T', _, _] => SegmentType::Sensor,
        _ => SegmentType::Structural,
    }
}

// 2. Lettura Morfologica (La Forma)
pub fn calculate_codon_radius(codon: &[char]) -> f32 {
    let sum: f32 = codon
        .iter()
        .map(|base| match base {
            'A' => 20.0,
            'C' => 40.0,
            'G' => 60.0,
            'T' => 80.0,
            _ => 30.0,
        })
        .sum();
    sum / codon.len() as f32
}

#[derive(Debug, Clone, Copy)]
pub struct FunctionalSegment {
    pub p1: Coor,
    pub p2: Coor,
    pub segment_type: SegmentType,
    // Possiamo salvare quanti codoni consecutivi hanno formato questo segmento!
    pub span_length: usize,
}
