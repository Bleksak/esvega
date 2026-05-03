mod shared;

mod animation;
mod attribute;
mod filter;
mod gradient;
mod link_media;
mod path;
mod presentation;

// Re-export everything that was previously public from attributes.rs
pub use animation::*;
pub use attribute::Attribute;
pub use filter::*;
pub use gradient::*;
pub use link_media::*;
pub use path::*;
pub use presentation::*;
