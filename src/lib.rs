extern crate eap_base;
pub use eap_base::*;

#[cfg(feature = "derive")]
extern crate eap_derive;
#[cfg(feature = "derive")]
pub use eap_derive::Config;

pub mod prelude {
    pub use eap_base::config::Config;
    pub use eap_base::environment::{Environment, Error};
}