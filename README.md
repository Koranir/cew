A personal utility crate.

Adds reexports of `snafu`, new piping traits (`Pipe`, `Inspect`, `Lay`), and reexports `tracing` + `tracing_subscriber` (as `tracing_utils::subscriber`) with a prelude and quick init function.

Snafu's `Whatever` type is reexported as `R = Result<T, Whatever>`, and the `R<()>` is reexported as `U`. Use `cew::e!()` to call `snafu::whatever!()` and `cew::me!()` to call `Err(snafu::whatever!())`.

The `Pipe`, `Inspect`, and `Lay` traits provide functions to reduce the amount of stacked parenthesis in long method chains, like the `tap` crate.

The `tracing_subscriber` quick init should be overriden with a custom impl if needed, but it gets you started quickly.

Usage:
```rs
use cew::prelude::*;

fn main() -> cew::U {
    cew::tracing::init_filtered_w_env(
      cew::tracing::fmt_layer().without_time(),
      // What should the default level be if the environment variable is not set?
      "info,verbose_crate=warn"
    ).whatever_context("Failed to initialise tracing")?;

    info!("Traced").

    cew::me!("Error")
}
```
