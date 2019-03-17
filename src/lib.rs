
pub mod rect;
pub mod qtree_node;

pub use rect::*;
pub use qtree_node::*;

#[cfg(feature = "ggez")]
pub type Point2 = ggez::graphics::Point2;

#[cfg(not(feature = "ggez"))]
pub type Point2 = nalgebra::Point2<f32>;
