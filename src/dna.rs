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
        // Usiamo unreachable invece di unimplemented, perché i controlli
        // nel costruttore garantiscono che avremo solo A, C, G o T.
        _ => unreachable!("Base non valida riscontrata nel DNA!"),
    }
}

impl Dna {
    pub fn get_compl(&self) -> Self {
        Self {
            // .map() applica elegantemente la funzione `complement` a tutti e 32 gli elementi
            sequence: self.sequence.map(complement),
            // I geni accesi/spenti rimangono esattamente gli stessi
            activity_mask: self.activity_mask,
        }
    }

    pub fn new(sequence: &str) -> Self {
        let len = sequence.len();
        assert!(
            len <= GENES_NUM,
            "Sequence must have len <= 32, given {}",
            len
        );
        assert!(
            sequence.chars().all(|x| BASES.contains(x)),
            "Sequence must contains only {}",
            BASES
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

use std::ops::Add;

// Implementiamo il "Crossover Genetico"
impl Add for &Dna {
    type Output = Dna;

    fn add(self, rhs: Self) -> Self::Output {
        let mut seq_a = String::new();
        let mut seq_b = String::new();

        // 1. Estrai i geni attivi del Genitore A (Sinistra)
        for i in 0..GENES_NUM {
            if (self.activity_mask >> i) & 1 == 1 {
                seq_a.push(self.sequence[i]);
            }
        }

        // 2. Estrai i geni attivi del Genitore B (Destra)
        for i in 0..GENES_NUM {
            if (rhs.activity_mask >> i) & 1 == 1 {
                seq_b.push(rhs.sequence[i]);
            }
        }

        // 3. Meiosi: Tagliamo a metà il corredo genetico.
        // Rust permette di usare le 'slice' in modo molto sicuro e veloce.
        let mid_a = seq_a.len() / 2;
        let mid_b = seq_b.len() / 2;

        let half_a = &seq_a[0..mid_a]; // Prende la prima metà di A
        let half_b = &seq_b[mid_b..]; // Prende dalla metà alla fine di B

        // 4. Ricombinazione
        let mut combined_sequence = format!("{}{}", half_a, half_b);

        // Controllo di sicurezza: se per qualche strano caso i frammenti sommati
        // superano 32 (non dovrebbe accadere con le divisioni, ma la prudenza non è mai troppa)
        if combined_sequence.len() > GENES_NUM {
            combined_sequence.truncate(GENES_NUM);
        }

        // Il nuovo organismo riceve la fusione esatta del 50% di entrambi
        Dna::new(&combined_sequence)
    }
}

impl Add for Dna {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}
