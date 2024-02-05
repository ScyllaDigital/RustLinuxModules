pub(crate) extern crate core;
pub(crate) extern crate no_panics_whatsoever;

#[allow(unused_imports)]
pub(crate) use core::{
  clone::Clone,
  cmp::{Eq, Ord, PartialEq, PartialOrd},
  convert::{AsMut, AsRef, From, TryFrom},
  default::Default,
  fmt::{Debug, Display},
  future::{Future, IntoFuture},
  iter::{IntoIterator, Iterator},
  option::Option,
  result::Result,
  time::Duration,
};
