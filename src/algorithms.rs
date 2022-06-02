pub mod search;
pub mod sort;

pub mod examples {
    use rand::{prelude::SliceRandom, thread_rng};

    pub fn run_binary_search() {
        use crate::algorithms::search::binary_search;
        let needle = 'd';
        let haystack: Vec<char> = ('a'..='z').collect();
        dbg!(binary_search::run(needle, haystack));
    }

    pub fn run_bubble_sort() {
        use crate::algorithms::sort::bubble_sort;
        let mut list: Vec<usize> = (0..=10).collect();
        list.shuffle(&mut thread_rng());
        dbg!(&list);
        dbg!(bubble_sort::run(list));
    }
}
