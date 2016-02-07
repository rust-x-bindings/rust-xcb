# Rust XCB

[![Build Status](https://travis-ci.org/rtbo/rust-xcb.svg?branch=master)](https://travis-ci.org/rtbo/rust-xcb)

Rust-XCB is a set of bindings and wrappers for [XCB](http://xcb.freedesktop.org). It uses the XML
protocol descriptions from XCB to generate the bindings and the wrappers.

Rust-XCB is only intended as an interface to XCB, so provides nothing above and beyond that.

```toml
[dependencies]
xcb = "0.4.1"
```

## The bindings

The bindings are generated from the `rs_client.py` script with help from the `xcbgen` library (also
from XCB). The bindings are inside the `ffi` module, which also contains the hand-written bindings
to the core library. However, all of the bindings are still in the repository, so running it is not
necessary.

Bindings reflect the C API almost one for one.

## The wrapper

The wrappers are generated from the same files, and provide a safe and more convenient wrapper over
the low-level bindings by having automatic destructors for returned data, trait implementations for
object "types" and other safe helpers.

## Changelog

 * 0.4.1
 rtbo - 2016
    - generating union accessors
    - handling of bool parameters in the wrapper API
    - Travis

 * 0.4.0
 rtbo/laumann - 2016
    - first fully functional wrappers
    - rewritten rs_client.py
    - new examples
    - made ffi very close to C
    - fixed wrappers segfaults

 * 0.3.0
 Aatch - 2013
