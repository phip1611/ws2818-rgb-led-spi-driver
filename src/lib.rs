mod setup;
pub use setup::setup_spi;
// Reexport
pub use setup::Spidev;

pub mod timings;
pub mod util;
pub mod encoding;
