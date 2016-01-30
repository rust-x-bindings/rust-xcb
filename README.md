# Rust XCB

Rust-XCB is a set of bindings and wrappers for [XCB](http://xcb.freedesktop.org). It uses the XML
protocol descriptions from XCB to generate the bindings and the wrappers.

Rust-XCB is only intended as an interface to XCB, so provides nothing above and beyond that.

## The bindings

The bindings are generated from the `rs_client.py` script with help from the `xcbgen` library (also
from XCB). The bindings are inside the `ll` module, which also contains the hand-written bindings
to the core library. However, all of the bindings are still in the repository, so running it is not
necessary.

## The wrapper

The wrappers are generated from the same files, and provide a safe and more convenient wrapper over
the low-level bindings by having automatic destructors for returned data, trait implementations for
object "types" and other safe helpers.

## To Do

The issues tracker has more details, but the high-level things that need to be done are:

* Generate Intuitive wrappers. (#1)
* Get unions generating properly. (#2)
* Write Tests. (#3)
