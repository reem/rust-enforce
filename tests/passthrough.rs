#![feature(phase)]
#[phase(plugin)] extern crate stainless;
#[phase(plugin, link)] extern crate enforce;

describe! passthrough {
    describe! to { it "does nothing" { let x = enforce!(7u); assert_eq!(x.to(), x) } }
    describe! bee { it "does nothing" { let x = enforce!(7u); assert_eq!(x.bee(), x) } } 
    describe! a { it "does nothing" { let x = enforce!(7u); assert_eq!(x.a(), x) } }
    describe! an { it "does nothing" { let x = enforce!(7u); assert_eq!(x.an(), x) } }
    describe! at { it "does nothing" { let x = enforce!(7u); assert_eq!(x.at(), x) } }
    describe! have { it "does nothing" { let x = enforce!(7u); assert_eq!(x.have(), x) } }
    describe! is { it "does nothing" { let x = enforce!(7u); assert_eq!(x.is(), x) } }
}

