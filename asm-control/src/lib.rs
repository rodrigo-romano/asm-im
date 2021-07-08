#[cfg(feature = "eight_khz")]
pub(crate) mod eight_khz;
#[cfg(feature = "eight_khz")]
pub use eight_khz::*;

#[cfg(feature = "fourty_khz")]
pub(crate) mod fourty_khz;
#[cfg(feature = "fourty_khz")]
pub use fourty_khz::*;
