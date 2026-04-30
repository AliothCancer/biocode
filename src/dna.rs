pub mod bitwise_utils;

use macroquad::prelude::rand;

use crate::{A, BASES, BitFlagDim, C, G, GENES_NUM, T, get_shuffled_indices};

#[derive(Debug, Clone)]
pub struct Dna {
    pub sequence: [Base; GENES_NUM],
    pub activity_mask: BitFlagDim,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Base {
    A,
    C,
    G,
    T,
}

impl Base {
    pub fn from_char(ch: char) -> Self {
        match ch {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            _ => unreachable!("Base non valida riscontrata nel DNA!"),
        }
    }
}

fn complement(base: Base) -> Base {
    match base {
        A => T,
        C => G,
        G => C,
        T => A,
    }
}

impl Dna {
    pub fn get_active_bases(&self) -> &[Base] {
        &self.sequence[0..self.activity_mask.count_ones() as usize]
    }
    pub fn seq_from_str(sequence: &str) -> [Base; GENES_NUM] {
        let mut seq_array = [A; GENES_NUM];
        for (n, i) in sequence.chars().map(Base::from_char).enumerate() {
            seq_array[n] = i;
        }
        seq_array
    }
    pub fn get_compl(&self) -> Self {
        Self {
            sequence: self.sequence.map(complement),
            activity_mask: self.activity_mask,
        }
    }

    pub fn with_string_mask(sequence: &str, mask_str: &str) -> Self {
        assert!(sequence.len() <= GENES_NUM);

        Self {
            sequence: Self::seq_from_str(sequence),
            activity_mask: BitFlagDim::from_str_radix(mask_str, 2).unwrap_or(0),
        }
    }

    pub fn new(sequence: &str) -> Self {
        let len = sequence.len();
        assert!(
            len <= GENES_NUM,
            "dna sequence len exceeded maximum ({}), invalid value: {}",
            GENES_NUM,
            len
        );

        let mut seq_array = [A; GENES_NUM];
        let mut activity_mask = 0 as BitFlagDim;

        let indices = get_shuffled_indices();
        let mut char_iter = sequence.chars().map(Base::from_char);

        for (i, &pos) in indices.iter().enumerate() {
            if i < len {
                seq_array[pos] = char_iter.next().unwrap();
                activity_mask |= 1 << pos;
            } else {
                seq_array[pos] = BASES[rand::gen_range(0, 4) as usize];
            }
        }

        Self {
            sequence: seq_array,
            activity_mask,
        }
    }
}
