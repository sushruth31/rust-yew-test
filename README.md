# yew-app — a Rust/WebAssembly SPA: to-do list + PokéAPI browser

A two-page single-page app written in Rust and compiled to WebAssembly with
[Yew](https://yew.rs): a client-side to-do list and a page that pulls random pokémon
off a public REST API. Built over a couple of weekends to find out what a React-shaped
component model feels like when the language underneath has no `null`, no exceptions and
a borrow checker. The interesting part is the seam — how much of a UI app can be kept in
plain, host-testable Rust while `wasm32` stays a thin rendering shell.

## Stack

| Piece | Why |
| --- | --- |
| Yew 0.21 (`csr`) | Function components + hooks; closest thing to React in Rust |
| yew-router 0.18 | History-API routing with typed routes — a bad path is a compile error |
| gloo-net 0.5 | `fetch` wrapper sized for wasm; `reqwest` drags a native HTTP stack in |
| serde / serde_json | Typed decode of a ~200 KB PokéAPI payload down to two fields |
| thiserror | Domain errors as an enum, so the view matches instead of unwrapping |
| Trunk | Bundler: builds the wasm, hashes it, writes `dist/`, serves with reload |

## Running it

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk

cp .env.example .env
set -a && . ./.env && set +a     # POKEAPI_BASE_URL; the build fails without it

trunk serve                      # http://127.0.0.1:8080
trunk build --release            # -> dist/
```

Tests need no wasm toolchain and no browser:

```bash
cargo test
```

## Architecture

```
                    build.rs ── validates POKEAPI_BASE_URL ──► config::POKEAPI_BASE_URL
                                                                        │
  src/todo.rs        pure rules: add / toggle / remove, ids, counters   │
  src/pokeapi.rs     pure rules: endpoint building, JSON contract  ◄────┘
        ▲  no yew, no web-sys, no HTTP — builds and tests on the host
        │
        │  #[cfg(target_arch = "wasm32")]
        ▼
  src/ui/todo.rs     impl Reducible for TodoList  → use_reducer, renders rows
  src/ui/pokemon.rs  gloo-net fetch → pokeapi::parse → Idle | Loading | Failed
  src/ui/navbar.rs   <Link<Route>> anchors
  src/ui/route.rs    Route enum (Routable)
  src/ui/mod.rs      <App>: BrowserRouter + Switch
  src/main.rs        wasm entrypoint; on the host it exits with a hint
```

| Module | Lines | Tested |
| --- | --- | --- |
| `todo` | 88 | 9 tests |
| `pokeapi` | 82 | 9 tests |
| `ui/*` | 222 | rendering only |

## Design notes

- **The dependency graph is split by target, not just the code.** Yew, web-sys and
  gloo-net live under `[target.'cfg(target_arch = "wasm32")'.dependencies]`, so a host
  build resolves 22 crates instead of the 107 the browser build needs. Consequence:
  `cargo test` is 4.4 s cold and 0.09 s warm, with no `wasm-pack`, no headless browser
  and no `wasm-bindgen-test`. That constraint is what forced the rules out of the
  components in the first place — the domain modules physically cannot reach for a hook.
- **Items are addressed by a monotonic id, never by their rendered text.** The first
  version read `event.target().inner_html()` to work out which row was clicked. That ties
  behaviour to markup, and it silently does the wrong thing with two identically-named
  items. Ids are also never recycled after a removal, so a callback captured by an
  earlier render resolves to nothing instead of hitting whatever now occupies that slot.
  There is a test named after exactly that case.
- **`sprites.back_default` is nullable, and `unwrap()` in wasm takes the page with it.**
  A panic aborts the module; there is no error boundary to catch it, so one unlucky
  pokémon leaves a blank screen. `pokeapi::parse` returns `Result`, falls back
  back-sprite → front-sprite → `PokeError::NoSprite`, and the page renders a `Failed`
  state. Every fallible step in the fetch path is a `?`, not an `unwrap`.
- **Configuration is resolved at build time because wasm has no process environment.**
  `build.rs` reads `POKEAPI_BASE_URL`, rejects anything that is not an absolute http(s)
  URL, strips a trailing slash and re-exports it via `cargo:rustc-env`. Missing config
  fails the build naming the variable, rather than shipping a bundle that fetches from
  `undefined/pokemon/42`.
- **Releasing a pokémon is positional, not by name.** Drawing from a 151-wide dex, the
  chance of a duplicate species passes 50 % by the fifteenth catch (birthday problem);
  the original `filter(|p| p.name != name)` deleted both copies. Elsewhere `toggle` and
  `remove` are O(n) scans over a `Vec` — for a hand-typed list an index would cost more
  in indirection than it saves, and insertion order is free.
- **Navigation uses `<Link<Route>>` rather than `use_navigator().unwrap()` + `onclick`.**
  It removes the one remaining unwrap, and real anchors keep middle-click, ctrl-click and
  copy-link-address working — behaviour a button handler quietly destroys.

Release bundle: 673 KB of wasm, 213 KB gzipped.

## Tests

18 tests over the two pure modules, run with `cargo test` (host target, no browser).

`tests/todo.rs` — whitespace-only input rejected rather than stored; text trimmed;
duplicate labels get distinct ids so removing one keeps the other; ids never reused after
a removal; toggle is its own inverse; unknown ids are no-ops instead of panics;
`remaining()` counts only unfinished items; insertion order survives edits.

`tests/pokeapi.rs` — back sprite preferred, front sprite as fallback, both-null is an
error and not a panic; missing `sprites` object and an HTML error page served with a 200
are both rejected; unknown JSON fields ignored; endpoint building is idempotent under
trailing slashes; `random_id` stays in `1..=151` and reaches both ends (20 000 seeded
draws — the original `gen_range(1..100)` could never return 100); display names
title-case each hyphenated segment (`nidoran-f`, `mr-mime`).
