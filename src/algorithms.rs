pub mod search;

pub mod examples {
    pub fn run_binary_search() {
        use crate::algorithms::search::binary_search;
        let needle = 'd';
        let haystack: Vec<char> = ('a'..='z').collect();
        dbg!(binary_search::run(needle, haystack));
    }
}
