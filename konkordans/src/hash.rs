

pub fn hash_three(word: &str) -> usize {
    let chars = word.chars();

    let mut hash: usize = 0;
    let mut n = 0;
    for c in chars {
        if n == 3 {
            break;
        }
        hash = hash.wrapping_mul(17).wrapping_add(c as usize);
        n += 1;
    }

    hash
}