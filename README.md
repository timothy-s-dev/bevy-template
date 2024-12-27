# Bevy Template

A base template to be used for future bevy projects. Features include:

* Placeholder splashscreen, main menu, and game states
* Asset hot-reloads
* bevy_kira_audio
* A handful of cargo command shortcuts (see below)
* Schedule graph rendering via bevy_mod_debugdump (see below)


## Building / Running

Local builds can be run with `cargo dev`. This includes asset hot-reloads, but does not watch code files for changes.

A release build can be prepared with `cargo release`. This is quite slow, but includes a ton of compiler optimizations
to improve run-time performance.

The codegen crate, which will be used to convert raw data files into Rust enums/structs/etc, can be run with
`cargo codegen`. This doesn't currently do anything, as the crate is basically empty.

The system schedule graph can be found in the `docs` directory, and can be automatically updated via the shell script:
`./update_schedule_graph.sh`. It currently outputs the graph for all systems in the `Update` schedule, but could be
adjusted to do otherwise later, if that proves valuable.


## Project Structure

The project is split into three crates:

 * `data` - Contains `Components` and `Resources`
 * `game` - Contains `Systems`, the `App` definition, and other logic
 * `codegen` - Converts raw data files into Rust types/data for use in the other crates

The `data` and `game` crates are going to be further divided into modules based on `GlobalState`s at the top level,
then by sub-states or concepts further down.

The crates make heavy use of the `prelude` pattern, and the `prelude` in the `game` crate re-exports the `preludes` of
both the `data` crate, as well as several other dependencies (`bevy`, `bevy_kira_audio`, etc.) As such, a single
`use crate::prelude::*` at the top of a file is often sufficient to include everything needed.
