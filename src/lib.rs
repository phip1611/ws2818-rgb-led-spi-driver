mod setup;
pub use setup::setup_spi;

pub mod timings;
pub mod encoding;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
