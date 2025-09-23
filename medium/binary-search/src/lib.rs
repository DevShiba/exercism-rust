pub fn find(array: &[i32], key: i32)-> Option<usize> {
    let mut low = 0;
    let mut high = array.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match array[mid].cmp(&key){
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}