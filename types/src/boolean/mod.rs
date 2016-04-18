//! Implementation of the Elasticsearch `boolean` types.
//!
//! # Examples
//!
//! Map with a default `boolean`:
//!
//! ```
//! struct MyType {
//! 	pub field: bool
//! }
//! ```
//!
//! Map with a custom `boolean`:
//!
//! ```
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::boolean::prelude::*;
//! # #[derive(Debug, Clone, Default)]
//! # pub struct MyBooleanMapping;
//! # impl ElasticBooleanMapping for MyBooleanMapping { }
//! # impl_boolean_mapping!(MyBooleanMapping);
//! struct MyType {
//! 	pub field: ElasticBoolean<MyBooleanMapping>
//! }
//! # }
//! ```

mod boolean;

pub mod mapping;
pub use self::boolean::*;

pub mod prelude {
	//! Includes non-mapping types for the `boolean` type.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::boolean::*;
}
