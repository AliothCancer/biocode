use macroquad::prelude::rand;
use std::ops::Add;

pub const GENES_NUM: usize = 32;
pub const BASES: &str = "ACGT";

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

// ---------------------------------------------------------
// HELPER: Genera un array di 32 indici mescolati sullo Stack
// ---------------------------------------------------------
fn get_shuffled_indices() -> [usize; GENES_NUM] {
    let mut indices = [0; GENES_NUM];
    (0..GENES_NUM).for_each(|i| {
        indices[i] = i;
    });

    // Algoritmo Fisher-Yates
    for i in (1..GENES_NUM).rev() {
        let j = rand::gen_range(0, i + 1) as usize;
        indices.swap(i, j);
    }

    indices
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

// ---------------------------------------------------------
// NUOVA MEIOSI (Crossover Uniforme posizionale)
// ---------------------------------------------------------
impl Add for Dna {
    type Output = Dna;

    fn add(self, rhs: Self) -> Self::Output {
        let mut child_seq = ['A'; GENES_NUM];
        let mut child_mask = 0_u32;

        // Richiamiamo la funzione helper per avere un nuovo set di indici mescolati
        let indices = get_shuffled_indices();

        for (i, &pos) in indices.iter().enumerate() {
            if i < (GENES_NUM / 2) {
                child_seq[pos] = self.sequence[pos];
                if (self.activity_mask >> pos) & 1 == 1 {
                    child_mask |= 1 << pos;
                }
            } else {
                child_seq[pos] = rhs.sequence[pos];
                if (rhs.activity_mask >> pos) & 1 == 1 {
                    child_mask |= 1 << pos;
                }
            }
        }

        Dna {
            sequence: child_seq,
            activity_mask: child_mask,
        }
    }
}
