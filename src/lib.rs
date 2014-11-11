#![feature(macro_rules)]
#![license = "MIT"]
#![deny(missing_docs)]
#![deny(warnings)]

//! Fluid assertions in Rust.

use std::fmt::{mod, Show};

#[macro_export]
macro_rules! enforce {
    ($e:expr) => {
        ::enforce::Enforce {
            data: $e,
            repr: stringify!($e),
            location: (file!(), line!()),
            negated: false
        }
    }
}

/// A wrapper around a piece of data that enables assertions.
#[deriving(PartialEq)]
pub struct Enforce<T: Show> {
    #[doc(hidden)]
    pub data: T,

    #[doc(hidden)]
    pub repr: &'static str,

    #[doc(hidden)]
    pub location: (&'static str, uint),

    #[doc(hidden)]
    pub negated: bool
}

impl<T: Show> Show for Enforce<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T: Show> Enforce<T> {
    /// No-Op for fluid chains
    pub fn to(self) -> Enforce<T> { self }

    /// No-Op for fluid chains
    pub fn bee(self) -> Enforce<T> { self }

    /// No-Op for fluid chains
    pub fn a(self) -> Enforce<T> { self }

    /// No-Op for fluid chains
    pub fn an(self) -> Enforce<T> { self }

    /// No-Op for fluid chains
    pub fn at(self) -> Enforce<T> { self }

    /// No-Op for fluid chains
    pub fn have(self) -> Enforce<T> { self }

    /// No-Op for fluid chains
    pub fn is(self) -> Enforce<T> { self }

    /// Inverts all assertions.
    ///
    /// Can be applied multiple times for greater fun.
    ///
    /// ```ignore
    /// enforce!(7u).is().not().equal(8);
    /// ```
    pub fn not(mut self) -> Enforce<T> {
        self.negated = !self.negated;
        self
    }

    fn error(&self, msg: String) -> ! {
        let (file, line) = self.location;
        panic!("\nEnforce Error {}:{} - {}", file, line, msg);
    }
}

impl<T: PartialEq + Show> Enforce<T> {
    /// Asserts that the value inside `enforce!` and the passed-in value are equal.
    pub fn equal(self, val: T) {
        if self.negated {
            if self.data == val && val == self.data {
                self.error(format!("{} == {}", self.repr, val));
            }
        } else {
            if self.data != val || val != self.data {
                self.error(format!("{} != {}", self.repr, val));
            }
        }
    }

    /// Alias for equal
    pub fn same(self, val: T) { self.equal(val) }

    /// Alias for equal
    pub fn eql(self, val: T) { self.equal(val) }

    /// Alias for equal
    pub fn equivalent(self, val: T) { self.equal(val) }
}

impl<T: Show> Enforce<Option<T>> {
    /// Asserts that the `Option` inside of `enforce!` is `Some`.
    pub fn some(mut self) {
        if self.negated { self.negated = false; return self.none(); }

        if !self.data.is_some() {
            self.error(format!("{} is None", self.repr));
        }
    }

    /// Asserts that the `Option` inside of `enforce!` is `None`.
    pub fn none(mut self) {
        if self.negated { self.negated = false; self.some(); return; }

        if !self.data.is_none() {
            self.error(format!("{} is {}", self.repr, self.data));
        }
    }
}

impl<S: Show, E: Show> Enforce<Result<S, E>> {
    /// Asserts that the `Result` inside of `enforce!` is `Ok`.
    pub fn ok(mut self) {
        if self.negated { self.negated = false; return self.err(); }

        if !self.data.is_ok() {
            self.error(format!("{} is {}", self.repr, self.data));
        }
    }

    /// Asserts that the `Result` inside of `enforce!` is `Err`.
    pub fn err(mut self) {
        if self.negated { self.negated = false; return self.ok(); }

        if !self.data.is_err() {
            self.error(format!("{} is {}", self.repr, self.data));
        }
    }

}

