# DelayWriter

`DelayWriter` provides a convenient way to output text with a delay after each newline character (`\n`). It wraps a writer and introduces a delay after each newline, giving an animated effect to the text displayed.

## Install

Add `delay_writer` in your `Cargo.toml`:

```toml
[dependencies]
delay_writer = "0.1.0"
```

## Usage

```rust
use std::{time::Duration, io::Write};
use delay_writer::DelayWriter;

let stdout = std::io::stdout();
let mut writer = DelayWriter::new(stdout, Duration::from_millis(100));

let text = "Hello\nWorld!";

writer.write_all(text.as_bytes()).unwrap();
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
