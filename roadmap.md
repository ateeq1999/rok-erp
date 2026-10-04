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

**Goal:** one command installs every module the pharmacy needs on a fresh PostgreSQL, safely
and repeatably.

- [x] Step 1: the eight modules copied from `rok-pos-database/modules` into `database/modules`.
- [x] Step 2: the module installer, `rok-pos-database/src/module_installer.rs` - our own, not
      `Db::migrate`, because module migrations reuse numbers such as `0001`:
  - [x] Read every `module.toml` (serde + `toml`), sort by `depends_on`, refuse cycles.
  - [x] Run each pending `NNNN_*.up.sql` in its own transaction with `rok_db::raw`.
  - [x] Record it in `core.applied_migrations` with a SHA-256 checksum.
  - [x] Refuse to start if a shipped migration's checksum changed.
  - [x] Upsert `[[permissions]]` into `core.permissions`.
  - [x] Write progress to `core.module_jobs.steps`.
- [x] Step 3: `database/modules/pharmacy/migrations/0002_add_clinical_stock_and_compliance`:
      fifteen tables plus `claim_batch_id` on `pharmacy.insurance_claims`, with its `.down.sql`,
      and the six new permissions in `pharmacy/module.toml`.
- [x] Step 4: rok-db models in `rok-pos-database/src/models/`, one file per table group, with
      the `check` constraints' values as `DbEnum`s stored as text.
- [x] Step 5: `database/seeds/afya_pharmacy_story.sql` - both branches and the staff, the
      medicines and batches, RX-2210 to RX-2219, the controlled register at 80, UZ-7781,
      RC-0047 and both insurance batches. It loads twice without doubling anything.
- [x] Step 6: `tools/migration-verifier` - all ups, all downs, ups again, then a fourth round
      that must change nothing; wired into CI after the tests.
- [x] Step 6: `rok-pharmacy --install` - installs the eight modules in order on an empty
      database and loads the story seed; a second run applies nothing.
- [x] Tests: the installer runs twice with no changes; a changed migration is refused before
      any DDL runs; every down reverses its up and re-applies; row level security is checked on
      all fifteen new tables as `rok_pos_app`, which owns no tables and has no `BYPASSRLS`.

**Phase 1 is done:** `rok-pharmacy --install` on an empty database installs the eight modules
in order and loads the story seed, and CI's verifier passes.