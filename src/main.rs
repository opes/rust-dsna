mod algorithms;
mod structures;
pub use crate::algorithms::examples::{run_binary_search, run_bubble_sort};
pub use crate::structures::examples::run_collections;

fn main() {
    // Data Structures
    run_collections();

    // Algorithms
    run_binary_search();
    run_bubble_sort();
}
