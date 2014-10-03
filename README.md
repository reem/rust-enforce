# Enforce

> Fluid Assertions in Rust

## Example

*Uses [stainless](https://github.com/reem/stainless) for testing syntax*

```rust
describe! vec {
    describe! push {
        before_each {
            let mut v = vec![1u, 2, 3];
        }

        it "should increase the length by 1" {
            let old = v.len();
            v.push(4u);
            // Fluid chain with informative error messages.
            (enforce!(old + 1)).is().equal(v.len());
        }

        it "should add another value to the end of the vec" {
            v.push(4u);
            // Has `some`, `none`, `ok`, and `err` methods for checking Option
            // and Result APIs.
            (enforce!(*v.get_mut(3))).is().some();
        }
    }
}
```

### Failed Assertion

```
Enforce Error file.rs:17 - map.find("no such key") is None
```

## License

MIT

