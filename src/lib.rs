#![feature(macro_rules)]
#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]

//! Fluid assertions in Rust.

use std::fmt::Show;

#[macro_export]
macro_rules! enforce {
    ($e:expr) => {
        enforce::Enforce {
            data: $e,
            repr: stringify!($e),
            location (file!(), line!()),
            negated: false
        }
    }
}

pub struct Enforce<T: Show> {
    data: T,
    repr: &'static str,
    location: (&'static str, uint),
    negated: bool
}

impl<T: Show> Enforce<T> {
    pub fn to(self) -> Enforce<T> { self }
    pub fn bee(self) -> Enforce<T> { self }
    pub fn a(self) -> Enforce<T> { self }
    pub fn an(self) -> Enforce<T> { self }
    pub fn at(self) -> Enforce<T> { self }
    pub fn have(self) -> Enforce<T> { self }
    pub fn is(self) -> Enforce<T> { self }

    pub fn not(mut self) -> Enforce<T> {
        self.negated = !self.negated;
        self
    }

    fn error(&self, msg: String) -> ! {
        let (file, line) = self.location;
        fail!("\nEnforce Error {}:{} - {}", file, line, msg);
    }
}

impl<T: Eq + Show> Enforce<T> {
    pub fn equal(self, val: T) {
        if self.data != val || val != self.data {
            if self.negated {
                self.error(format!("{} == {}", self.repr, val));
            } else {
                self.error(format!("{} != {}", self.repr, val));
            }
        }
    }

    pub fn same(self, val: T) { self.equal(val) }
    pub fn eql(self, val: T) { self.equal(val) }
    pub fn equivalent(self, val: T) { self.equal(val) }
}

impl<T: Show> Enforce<Option<T>> {
    pub fn none(self) {
        if self.negated { self.some(); return; }

        if !self.data.is_none() {
            self.error(format!("{} is {}", self.repr, self.data));
        }
    }

    pub fn some(self) {
        if self.negated { return self.none(); }

        if !self.data.is_some() {
            self.error(format!("{} is None", self.repr));
        }
    }
}

impl<S: Show, E: Show> Enforce<Result<S, E>> {
    pub fn err(self) {
        if self.negated { return self.ok(); }

        if !self.data.is_err() {
            self.error(format!("{} is {}", self.repr, self.data));
        }
    }

    pub fn ok(self) {
        if self.negated { return self.err(); }

        if !self.data.is_ok() {
            self.error(format!("{} is {}", self.repr, self.data));
        }
    }
}

