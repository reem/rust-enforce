#![feature(phase)]
#![deny(warnings)]
#![allow(non_snake_case)]
#[phase(plugin)] extern crate stainless;
#[phase(plugin, link)] extern crate enforce;

describe! error_types {
    describe! option {
        describe! some {
            it "should check if something is Some" {
                (enforce!(Some(7u))).some();
            }

            failing "should fail if something is None" {
                (enforce!(None::<uint>)).some();
            }

            it "should support negation" {
                (enforce!(None::<()>)).not().some()
            }
        }

        describe! none {
            it "should check if something is None" {
                (enforce!(None::<()>)).none();
            }

            failing "should fail if something is Some" {
                (enforce!(Some(7u))).none();
            }

            it "should support negation" {
                (enforce!(Some(7u))).not().none()
            }
        }
    }

    describe! result {
        describe! ok {
            it "should check if something is Ok" {
                (enforce!(Ok::<uint, ()>(7u))).ok();
            }

            failing "should fail is something is Err" {
                (enforce!(Err::<(), uint>(7u))).ok();
            }

            it "should support negation" {
                (enforce!(Err::<(), ()>(()))).not().ok()
            }
        }

        describe! err {
            it "should check if something is Err" {
                (enforce!(Err::<(), uint>(7u))).err();
            }

            failing "should fail if something is Ok" {
                (enforce!(Ok::<uint, ()>(7u))).err();
            }

            it "should support negation" {
                (enforce!(Ok::<(), ()>(()))).not().err()
            }
        }
    }
}

