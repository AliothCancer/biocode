pub mod bitwise_utils;

use macroquad::prelude::rand;

use crate::{GENES_NUM, get_shuffled_indices};

#[derive(Debug, Clone, Copy)]
pub struct Dna {
    pub sequence: [char; GENES_NUM],
    pub activity_mask: u32,
}

fn complement(base: char) -> char {
    match base {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => unreachable!("Base non valida riscontrata nel DNA!"),
    }
}

impl Dna {
    pub fn get_compl(&self) -> Self {
        Self {
            sequence: self.sequence.map(complement),
            activity_mask: self.activity_mask,
        }
    }

    pub fn with_string_mask(sequence: &str, mask_str: &str) -> Self {
        assert!(sequence.len() <= GENES_NUM);

        let mut seq_array = ['A'; GENES_NUM];
        for (n, i) in sequence.chars().enumerate() {
            seq_array[n] = i;
        }

        Self {
            sequence: seq_array,
            activity_mask: u32::from_str_radix(mask_str, 2).unwrap_or(0),
        }
    }

    pub fn new(sequence: &str) -> Self {
        let len = sequence.len();
        assert!(
            len <= GENES_NUM,
            "La sequenza DNA non può superare i 32 caratteri"
        );

        let mut seq_array = ['A'; GENES_NUM];
        let mut activity_mask = 0_u32;
        let bases = ['A', 'C', 'G', 'T'];

        // Usiamo la nostra nuova funzione helper!
        let indices = get_shuffled_indices();
        let mut char_iter = sequence.chars();

        for (i, &pos) in indices.iter().enumerate() {
            if i < len {
                seq_array[pos] = char_iter.next().unwrap();
                activity_mask |= 1 << pos;
            } else {
                seq_array[pos] = bases[rand::gen_range(0, 4) as usize];
            }
        }

        Self {
            sequence: seq_array,
            activity_mask,
        }
    }
}
