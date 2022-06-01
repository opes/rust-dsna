use std::cmp::Ordering;

pub fn run(needle: char, haystack: Vec<char>) -> Option<usize> {
    let mut size = haystack.len();
    let mut start = 0;
    let mut end = size;

    while start < end {
        let mid = start + size / 2;

        match haystack[mid].cmp(&needle) {
            Ordering::Equal => {
                return Some(mid);
            },
            Ordering::Greater => {
                end = mid;
            },
            Ordering::Less => {
                start = mid + 1;
            }
        }

        size = end - start;
    }

    None
}
