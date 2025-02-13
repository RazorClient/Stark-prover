#![warn(non_snake_case)]
#![warn(unused_imports)]
#![warn(dead_code)]

use tracing::{info, warn, error, debug, trace};
use stark_101::utils::setup_tracing;

fn main() {
    setup_tracing();

    trace!("This is a trace message.");
    debug!("This is a debug message.");
    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
}
