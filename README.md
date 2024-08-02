# bapiDMM
<hr>

![bapi dmm logo](./bapidmm.png)

A modern dmm\_suite for BYOND, written entirely in Rust.

This loads maps into the game really, REALLY fast. The main bottleneck is entirely on how long it takes BYOND to run /New.

It has a custom zero-copy dmm parser - [dmm-lite](crates/dmm-lite), written entirely using [winnow](https://github.com/winnow-rs/winnow).

See [bapi\_dmm\_reader.dm](crates/bapi-dmm-reader/dm/bapi-dmm_reader.dm) for the supporting DM code.

