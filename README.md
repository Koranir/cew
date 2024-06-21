A personal utility crate.

Adds reexports of `color_eyre`, piping traits (`Pipe`, `Inspect`, `Lay`), a `block_on` implementation for futures, reexports `thiserror`, and reexports `tracing` + `tracing_subscriber` (as `tracing_utils::subscriber`) with a prelude and quick init function.

Run `cew::init()` to initialize `color_eyre`

`cew::R` is short for `color_eyre::Result`

`cew::U` is short for `color_eyre::Result<()>`

`cew::e!(..)` is short for `color_eyre::eyre::eyre!(..)`

`cew::me!(..)` is short for `Err(color_eyre::eyre::eyre!(..))`

The `Pipe`, `Inspect`, and `Lay` traits provide functions to reduce the amount of stacked parenthesis in long method chains.

The `tracing_subscriber` quick init should be overriden with a custom impl if needed, but it get you started quickly.
