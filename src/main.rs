pub mod neuron;
pub mod start;
use log::{error, info, warn};
fn main() {
    match start::start() {
        Err(e) => log::error!("{:?}", e),
        _ => (),
    };
}
