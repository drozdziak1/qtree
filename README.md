#qtree
This crate implements a simple quad tree - a data structure useful for indexing
2D space. This implementation allows you to store rectangles representing your
objects' bounding boxes and query all object IDs which contain a given point.
The unique ID implementation comes from
[`snowflake`](https://crates.io/crates/snowflake).

The `ggez` feature lets you integrate and debug the quad tree easily from
[ggez](https://github.com/ggez/ggez).

# Usage
See `examples/` for a simple ggez program taking advantage of object-cursor
collision detection using this crate.
