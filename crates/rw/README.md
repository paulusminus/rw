[![`build-badge`]](https://github.com/paulusminus/rw/actions/workflows/rust.yml)
[![`mit-badge`]](https://opensource.org/licenses/MIT)

# rw

rw is a library that exposes a polymorphic reader and writer. Reader can take input from stdin or from a file. Write can output to stdout or to a file.


## stdin

```no_run
use rw::generic::reader::Reader;

// use stdin
let reader = Reader::default();
```

## stdout

```no_run
use rw::generic::reader::Writer;

// use stdin
let mut writer = Writer::default();
```

[`build-badge`]: https://github.com/paulusminus/rw/actions/workflows/rust.yml/badge.svg
[`mit-badge`]: https://img.shields.io/badge/License-MIT-yellow.svg
