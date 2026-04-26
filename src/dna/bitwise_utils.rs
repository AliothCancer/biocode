pub fn is_bit_on(activity_mask: u32, pos: usize) -> bool {
    ((activity_mask >> pos) & 1) == 1
}
