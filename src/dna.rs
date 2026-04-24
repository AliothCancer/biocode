pub const GENES_NUM: usize = 32;
pub const BASES: &str = "ACGT";

pub struct Dna {
    pub sequence: [char; GENES_NUM],
    pub activity_mask: u32,
}

impl Dna {
    pub fn new(sequence: &str) -> Self {
        let len = sequence.len();
        assert!(
            len <= GENES_NUM,
            "La sequenza DNA non può superare i 32 caratteri"
        );

        let mut seq_array = ['A'; GENES_NUM];
        for (n, i) in sequence.chars().enumerate() {
            seq_array[n] = i;
        }

        // Calcola la maschera: se abbiamo 'len' caratteri, accendiamo 'len' bit.
        let activity_mask = if len == GENES_NUM {
            u32::MAX // Eccezione per 32 bit: accende tutti i bit (0xFFFFFFFF)
        } else {
            (1_u32 << len) - 1 // Es: se len=4, 1<<4 = 16. 16-1 = 15 (0b00001111)
        };

        Self {
            sequence: seq_array,
            activity_mask,
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
            // Converte una stringa di "0" e "1" in un u32 indicando la base (2)
            activity_mask: u32::from_str_radix(mask_str, 2).unwrap_or(0),
        }
    }
}
