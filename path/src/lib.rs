#![doc(html_logo_url = "https://nical.github.io/lyon-doc/lyon-logo.svg")]
#![deny(bare_trait_objects)]

//! Data structures and traits to work with paths (vector graphics).
//!
//! To build and consume paths, see the [builder](builder/index.html) and
//! [iterator](iterator/index.html) modules.
//!
//! This crate is reexported in [lyon](https://docs.rs/lyon/).
//!
//! # Examples
//!
//! ```
//! # extern crate lyon_path;
//! # fn main() {
//! use lyon_path::Path;
//! use lyon_path::math::{point};
//! use lyon_path::builder::*;
//!
//! // Create a builder object to build the path.
//! let mut builder = Path::builder();
//!
//! // Build a simple path.
//! let mut builder = Path::builder();
//! builder.move_to(point(0.0, 0.0));
//! builder.line_to(point(1.0, 2.0));
//! builder.line_to(point(2.0, 0.0));
//! builder.line_to(point(1.0, 1.0));
//! builder.close();
//!
//! // Generate the actual path object.
//! let path = builder.build();
//!
//! for event in &path {
//!     println!("{:?}", event);
//! }
//! # }
//! ```
//!

pub use lyon_geom as geom;

#[cfg(feature = "serialization")]
#[macro_use]
pub extern crate serde;

mod events;
mod path_state;
mod path;
pub mod id_path;
pub mod iterator;
pub mod builder;

pub use crate::path::*;
pub use crate::events::*;
pub use crate::path_state::*;
pub use crate::geom::ArcFlags;
pub use crate::geom::math as math;

use std::ops::{Add, Sub};
use std::u32;
use std::fmt;
use math::Point;

pub type Index = u32;

/// The fill rule defines how to determine what is inside and what is outside of the shape.
///
/// See the SVG specification.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum FillRule {
    EvenOdd,
    NonZero,
}

/// A virtual vertex offset in a geometry.
///
/// The `VertexId`s are only valid between `GeometryBuilder::begin_geometry` and
/// `GeometryBuilder::end_geometry`. `GeometryBuilder` implementations typically be translate
/// the ids internally so that first `VertexId` after `begin_geometry` is zero.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct VertexId(pub Index);

impl VertexId {
    pub const INVALID: VertexId = VertexId(u32::MAX);

    pub fn offset(&self) -> Index { self.0 }

    pub fn to_usize(&self) -> usize { self.0 as usize }

    pub fn from_usize(v: usize) -> Self { VertexId(v as Index) }
}

impl Add<u32> for VertexId {
    type Output = Self;
    fn add(self, rhs: u32) -> Self {
        VertexId(self.0 + rhs)
    }
}

impl Sub<u32> for VertexId {
    type Output = Self;
    fn sub(self, rhs: u32) -> Self {
        VertexId(self.0 - rhs)
    }
}

impl From<u16> for VertexId {
    fn from(v: u16) -> Self { VertexId(v as Index) }
}
impl From<u32> for VertexId {
    fn from(v: u32) -> Self { VertexId(v) }
}
impl From<i32> for VertexId {
    fn from(v: i32) -> Self { VertexId(v as Index) }
}

impl From<VertexId> for u16 {
    fn from(v: VertexId) -> Self { v.0 as u16 }
}
impl From<VertexId> for u32 {
    fn from(v: VertexId) -> Self { v.0 }
}
impl From<VertexId> for i32 {
    fn from(v: VertexId) -> Self { v.0 as i32 }
}
impl From<VertexId> for usize {
    fn from(v: VertexId) -> Self { v.0 as usize }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct CtrlPointId(pub u32);
impl CtrlPointId {
    //pub(crate) const INVALID: Self = CtrlPointId(!0u32);
    pub fn to_usize(&self) -> usize { self.0 as usize }
}

impl fmt::Debug for CtrlPointId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct EndpointId(pub u32);
impl EndpointId {
    //pub(crate) const INVALID: Self = EndpointId(!0u32);
    pub fn to_usize(&self) -> usize { self.0 as usize }
}

impl fmt::Debug for EndpointId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

pub trait Vertex : Clone {
    fn position(&self) -> Point;
    fn set_position(&mut self, pos: Point);
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self;
}

impl Vertex for Point {
    fn position(&self) -> Point { *self }
    fn set_position(&mut self, pos: Point) { *self = pos; }
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self { a.lerp(*b, t) }
}
