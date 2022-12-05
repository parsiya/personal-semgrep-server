// diagnostic utilities
use std::fmt::Debug;

use log::info;

// print_vector logs the contents of a vector.
pub(crate) fn print_vector<T: Debug>(vec: Vec<T>) {
    for item in vec {
        info!("{:?}", item);
    }
}
