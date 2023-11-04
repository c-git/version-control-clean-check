# version-control-clean-check
Checks the status of the version control system.
Based on [cargo](https://github.com/rust-lang/cargo)'s [`check_version_control`](https://github.com/rust-lang/cargo/blob/4b84887848a31c6f83434cee2135f4fb0e2c9cf3/src/cargo/ops/fix.rs#L146). 
Used in cargo before possibly destructive changes are done like running `cargo fix`.

## Display Messages for Errors
Display messages for errors are meant to be human readable and as such are not considered a breaking change if they are changed for clarity.

## License

All code in this repository is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

<!-- TODO: Figure out solution to broken license links -->

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both as noted in
this [issue](https://github.com/bevyengine/bevy/issues/2373) on [Bevy](https://bevyengine.org)'s repo.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
