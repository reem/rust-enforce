#![feature(phase)]
#[phase(plugin)] extern crate stainless;
#[phase(plugin, link)] extern crate enforce;

describe! equal {
    describe! equal {
        it "should assert that two things are equal" {
            (enforce!(7u)).is().equal(7u);
        }

        it "should support negations" {
            (enforce!(7u)).is().not().equal(8u);
        }

        failing "should fail if two things are not equal" {
            (enforce!(7u)).is().equal(8u);
        }

        failing "should fail if two things are equal and negated" {
            (enforce!(7u)).is().not().equal(7u);
        }
    }

    describe! synonyms {
        describe! same {
            it "should have the same behavior as equal" {
                (enforce!(7u)).is().equal(7u);
                (enforce!(7u)).is().not().equal(8u);
            }
        }

        describe! eql {
            it "should have the same behavior as equal" {
                (enforce!(7u)).is().equal(7u);
                (enforce!(7u)).is().not().equal(8u);
            }
        }

        describe! equivalent {
            it "should have the same behavior as equal" {
                (enforce!(7u)).is().equal(7u);
                (enforce!(7u)).is().not().equal(8u);
            }
        }
    }
}

