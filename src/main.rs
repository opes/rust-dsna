mod algorithms;
mod structures;
pub use crate::algorithms::examples::run_binary_search;
pub use crate::structures::examples::run_collections;

fn main() {
    run_collections();
    run_binary_search();
}
