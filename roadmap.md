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
- [ ] Spikes, each a small test or example:
  - [ ] `Money` round trip with `rok_db::impl_value!`.
  - [ ] `in_business` with `with_tenant` and `set_config`, checked as the non-owner role.
  - [ ] `query::provide(cx, PharmacyDatabase(db))` and a `use_query` that lists rows.
  - [ ] One dosage label and one 80 mm receipt from `rok-pos-hardware`.

The spikes need `rok-db` and `rok-pos-hardware`, which are separate crates and come with their
own phases. Until then CI has no PostgreSQL service; the first query adds it.

## Phase 1 — Database and module installer

Nothing started.