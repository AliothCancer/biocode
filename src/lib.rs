pub mod cells;
pub mod coordinates;
pub mod dna;
pub mod segments;

use crate::dna::{Base, Dna, bitwise_utils::is_bit_on};
use macroquad::prelude::rand;

pub type BitFlagDim = u128;
pub const GENES_NUM: usize = 128;
pub const A: Base = Base::A;
pub const C: Base = Base::C;
pub const G: Base = Base::G;
pub const T: Base = Base::T;
pub const BASES: [Base; 4] = [A, C, G, T];

pub trait Breedable<T> {
    fn breed(&self) -> T;
}

// ---------------------------------------------------------
// NUOVA MEIOSI (Crossover Uniforme posizionale)
// ---------------------------------------------------------
impl Breedable<Dna> for (&Dna, &Dna) {
    fn breed(&self) -> Dna {
        let mut child_seq = [Base::A; GENES_NUM];
        let mut child_mask = 0 as BitFlagDim;

        // Richiamiamo la funzione helper per avere un nuovo set di indici mescolati
        let indices = get_shuffled_indices();

        for (i, &pos) in indices.iter().enumerate() {
            if i < (GENES_NUM / 2) {
                // pos is the randomized index
                child_seq[pos] = self.0.sequence[pos];
                if is_bit_on(self.0.activity_mask, pos) {
                    child_mask |= 1 << pos;
                }
            } else {
                child_seq[pos] = self.1.sequence[pos];
                if is_bit_on(self.1.activity_mask, pos) {
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

// impl Breedable<Cell> for (&Cell, &Cell) {
//     fn breed(&self) -> Cell {
//         Cell::new((&self.0.dna, &self.1.dna).breed(), self.0.center)
//     }
// }

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
