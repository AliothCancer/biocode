pub fn is_bit_on(activity_mask: u128, pos: usize) -> bool {
    ((activity_mask >> pos) & 1) == 1
}
