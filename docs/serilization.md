# Serilization

For serializing other types than just primitives, use the provided procedural macro provided like bellow

```rust
    #[derive(Serialize, Deserialize, Clone, PartialEq, TupleField)]
    struct YourStruct {
        x: i32,
        y: f64,
    }
```