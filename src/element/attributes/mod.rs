mod shared;

mod path;
mod presentation;
mod filter;
mod gradient;
mod link_media;
mod animation;
mod attribute;

// Re-export everything that was previously public from attributes.rs
pub use path::*;
pub use presentation::*;
pub use filter::*;
pub use gradient::*;
pub use link_media::*;
pub use animation::*;
pub use attribute::Attribute;
