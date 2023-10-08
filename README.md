
# send-sync-static

A very very basic crate, whose only purpose is to allow marking types `Send + Sync + 'static`. No dependencies, no nothing. That's it.

### Usage

The `SSS` trait is a shortcut for `Send + Sync + 'static`. Use it to mark a type as fulfilling these requirements. E.g.

```rust
pub async fn send_data<D>(data: D) where D: SSS {
   // Do something here
}
```

You can also use `FutureSSS` which is simply a shorthand for `Future + SSS`.

```rust
pub fn send_data<D: SSS>(data: D) -> impl FutureSSS {
   // Guarantees the async block is always Send, Sync, and 'static
    async move { 
        // Do something here
        drop(data)
    }
}
```

### License

Apache 2.0. This crate has so little code, it might not even be licensable.
