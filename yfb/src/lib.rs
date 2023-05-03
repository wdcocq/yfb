#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub mod binding;
pub mod components;
pub mod field;
pub mod hooks;
pub mod model;
pub mod modifier;
pub mod prelude;
pub mod state_model;

#[doc(inline)]
pub use binding::Binding;
#[doc(inline)]
pub use components::*;
#[doc(inline)]
pub use yfb_derive::Model;

#[doc(inline)]
pub use crate::model::Model;
