# Roadmap

Where `plan.md` says what to build, this file says where the build has got to. One line per
step, in plan order, with the plan's own wording where it helps.

## Phase 0 — Workspace, shell and spikes

**Goal:** an empty `rok-pharmacy` window that already looks like the boards.

- [x] Workspace: virtual manifest, `rust-toolchain.toml`, `rustfmt.toml`, workspace lints
      (`clippy::pedantic`, `missing_docs`, `unsafe_code = "deny"`), CI running fmt, clippy,
      docs and tests.
- [x] `rok-pos-domain`: `Money` over `Decimal`, held at the two places the database column
      keeps, with checked arithmetic and the board formats.
- [x] `rok-pos-shell`:
  - [x] `theme.rs`: `ThemePreset::Rok` with the pharmacy accent `#0F766E`, tint `#F0FDFA`, and
        text on tint `#115E59`, in light and dark, with contrast helpers.
  - [x] `fonts.rs`: Oxanium for numbers and codes, Outfit for text, both embedded.
  - [x] `app_frame.rs`: sidebar beside the route outlet, `Page` for the top bar and the page.
  - [x] `money_text.rs`, `batch_code_text.rs`: Oxanium, thousands separators, TZS without
        decimals, expiry shortened to a month and year.
- [x] `apps/rok-pharmacy`: `rok_ui::init`, embedded fonts, a `1440x900` window, the generated
      route tree, and every pharmacy route on a placeholder page.
- [x] Spikes, each a small test or example:
  - [x] `Money` round trip with `rok_db::impl_value!`: `rok-pos-domain/src/money_postgres.rs`
        holds the sqlx plumbing behind its `postgres` feature, because sqlx's traits are foreign
        to the crate that owns the type; `tests/money_round_trip.rs` writes and reads
        `numeric(18,2)`.
  - [x] `in_business` with `with_tenant` and `set_config`, checked as the non-owner role:
        `rok-pos-database/tests/tenant_isolation.rs` connects a second time as `rok_pos_app`,
        which owns no tables and has no `BYPASSRLS`, and reads through `sqlx::query` as well as
        `rok_db::raw`.
  - [x] `query::provide(cx, PharmacyDatabase(db))` and a `use_query` that lists rows:
        `rok-pos-pharmacy/tests/query_screen.rs` builds its own `Db` on `db::runtime()`, hands it
        over with `db::set_connection`, lists rows through `query::use_query`, and refreshes after
        `db::invalidate`.
  - [x] One dosage label and one 80 mm receipt from `rok-pos-hardware`: `escpos.rs` builds the
        bytes, `label.rs` puts a dosage label and a Code 128 batch barcode on them, and
        `printer.rs` writes them to a file, standard output or memory.

CI now runs a PostgreSQL 17 service and sets `DATABASE_URL`, so the spikes run rather than skip.
Without it they still skip rather than fail.

## Phase 1 — Database and module installer

Nothing started.