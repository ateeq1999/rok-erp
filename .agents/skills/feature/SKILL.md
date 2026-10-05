---
name: feature
description: Add or change a pharmacy feature (the feature-first BLoC contract).
---

# Feature contract

A feature is a screen that owns its domain, its data access, its state and its tests.
`features/dashboard` is the reference; copy its shape, not its figures.

Files:

- `src/features/<name>.rs` is the barrel: the four layers as `pub mod`, `#[cfg(test)] mod tests;`,
  and `pub use presentation::<Name>Page;` as the only item the rest of the app is given.
- `src/features.rs` holds `pub mod <name>;` and re-exports the page.
- `domain/` entities, enums, calculations. `data/` records, repository trait, the story source.
  `application/` events, state, use case, BLoC. `presentation/` styles, widgets, screen, page.
  `tests/` one file per layer. Barrel files, never `mod.rs`.

Dependency direction, enforced by what compiles:

```text
presentation -> application -> domain <- data
```

- `domain` imports no `gpui`, `rok_ui`, `rok_pos_shell`, `sqlx` or router. No `Dashboard`
  equivalent may know it will be drawn.
- `data` reaches down to `domain` only. Records are the source's own shape, with `From` into
  domain entities.
- `application` builds no elements, reads no source directly, and names no route. It moves
  state, nothing else.
- `presentation` never sees the repository, not even in a widget. The page is the only place
  an event is dispatched.

Domain:

- Entities own `Text = Arc<str>` for words drawn on every frame, so a re-render shares them.
  The data layer holds `String` and converts at the boundary.
- Money is `rok_pos_domain::Money`, never a float or a formatted string.
- Every rule is a free function over entities (`sales_total`, `payment_share`, `bar_heights`),
  `#[must_use]`, and returns a domain type. A rule that formats a figure for the screen has
  crossed into presentation; move it to a widget.
- A figure a screen cannot know is `MeasureValue` (Money, Count, Percent), not a `String` the
  widget has to take apart.
- Enums a `check` constraint already lists live in `domain/enums.rs` and nothing else.
- A rule asks what kind of thing the work is (is it done?), not how urgent it is drawn. A tone
  is a presentation hint; a `Done` marker is a fact.

BLoC:

- One `enum` of events (what happened) and one `enum` of states (what is on screen). The state
  has a variant per thing the screen draws differently: a refresh that empties the page needs
  its own, not a re-use of `Loading`.
- A repository trait with an RPITIT method returning `impl Future<Output = ..> + Send`. No
  `async_trait`, no `Box<dyn Future>`.
- The source's error type stops at the data layer. The BLoC maps it into the application error
  the screen draws.
- An empty result is a value the domain can hold (a branch with no other to compare against).
  A failed read is an error state. Do not return an empty board for a failure.
- `dispatch` is async and `&mut self`; `state()` and `branch_id()` are `#[must_use]`.

Presentation:

- `<Name>Page` is `#[component]`, holds the BLoC behind `cx.use_state`, and dispatches its
  first load exactly once. The route only frames it: `frame::page(path).child(<Name>Page::new())`.
- An event is dispatched on a copy of the BLoC and written back in one `entity.update`, because
  the entity cannot be borrowed across an await. The task is detached; dropping it cancels the
  read.
- The screen is one `match` over the state, one arm per variant, each returning a whole board.
  A widget receives figures and draws them; it does not re-derive them.
- `styles!` table, theme tokens only, in `presentation/styles.rs`. Widget modules are
  `pub(crate)`; nothing is re-exported past `presentation`.
- Money on screen through `rok_pos_shell::format_money`. A domain tone maps to a shell `Tone`
  in one helper.

Done means:

- The page is reachable only through `features::<Name>Page`; the old `screens/` module is gone.
- The domain rules have `#[test]`s against the story's own figures, including a zero case.
- The state machine has `#[gpui::test]`s with `cx.executor().block_test(..)`, covering a first
  read, a refresh that keeps the board, a failure, and a recovery.
- Every state the BLoC can leave is drawn in a window in `tests/widget_tests.rs`.
- `cargo fmt --all --check`, `RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets
  --all-features` and `cargo test --workspace --all-features` are clean.
- `roadmap.md` has the feature under its plan phase.
