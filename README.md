# toml-query

Work with [toml-rs]() `Value` objects in an easy way:

```rust
value.read("foo.bar.a.b.c") // -> Result<Value, Error>
value.set("foo.bar.a.b.c", 1) // -> Result<Value, Error>
value.set("foo.bar.a.b.c", Value::Integer(1)) // -> Result<Value, Error>
value.insert("foo.bar.a.b.c", Value::Integer(1)) // -> Result<bool, Error>
value.delete("foo.bar.a.b.c") // -> Result<bool, Error>
```

# License

MPL 2.0
