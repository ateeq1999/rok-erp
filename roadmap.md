# Roadmap

Where `plan.md` says what to build, this file says where the build has got to. One line per
step, in plan order, with the plan's own wording where it helps.

## Phase 0 — Workspace, shell and spikes

**Goal:** an empty `pharmacy` window that already looks like the boards.

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
- [x] `apps/pharmacy`: `rok_ui::init`, embedded fonts, a `1440x900` window, the generated
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

## Phase 13 - Dashboard, licences and reports

**Goal:** the owner and pharmacist in charge see the day and stay inspection-ready. Step 1,
the dashboard, is the feature the rest of the pharmacy's screens are being rebuilt as.

### Features are feature-first, with a BLoC and four layers

A screen owns its domain, its data access, its state machine and its tests, in
`crates/rok-pos-pharmacy/src/features/<name>/`:

```text
presentation -> application -> domain <- data
```

- [x] `features/dashboard/` is the reference, and the first feature built this way:
  - [x] `domain/`: `entities.rs` (`Dashboard`, `HourSales`, `Payment`, `MedicineSold`,
        `DashboardTask`, `OtherBranch`, and `Text = Arc<str>` for the words drawn every frame),
        `enums.rs` (`TaskKind`, `TaskSeverity`, `Destination`), `calculations.rs` (the rules:
        `sales_total`, `insurance`, `percentage`, `payment_share`, `bar_heights`, `tiles`,
        `branch_measures`, `best_sellers`, `open_tasks`), `measures.rs` (`MeasureValue`, so a
        widget never takes a figure apart again). No `gpui`, no router, no tones.
  - [x] `data/`: `models.rs` (the source's own records and `From` into the entities),
        `repository.rs` (the trait, with an RPITIT future rather than `async_trait`, and the
        data layer's own error), `story.rs` (the board's figures as records),
        `repository_impl.rs` (`StoryDashboardRepository`, plus an unavailable one so the error
        state can be drawn and tested).
  - [x] `application/`: `dashboard_event.rs`, `dashboard_state.rs` (`Initial`, `Loading`,
        `Refreshing { dashboard }`, `Loaded`, `Error`), `dashboard_use_cases.rs`,
        `dashboard_bloc.rs` (generic over the repository, `async dispatch`, `#[must_use] state`
        and `branch_id`).
  - [x] `presentation/`: `styles.rs`, `dashboard_screen.rs` (one match over the state, the route
        table and the figure formatting, which are presentation's own), `dashboard_page.rs` (the
        BLoC behind `cx.use_state`, the first load dispatched once, a read on a copy of the bloc
        written back in one update, the task detached), and `widgets/` for the six cards.
  - [x] `tests/`: `domain_tests.rs` (the board's figures, plus a day with nothing on it),
        `bloc_tests.rs` (first read, a refresh that keeps the board, a failure, a recovery, a
        branch change, the use case's error mapping), `widget_tests.rs` (every state drawn in a
        window, the figures the screen hands its widgets).
- [x] The dashboard route is now `frame::page("/").child(features::DashboardPage::new())`, and
      `screens/dashboard.rs` is gone: the page is reachable only through `features::DashboardPage`.
- [x] `features/prescriptions/`, the same four layers for the queue, whose rules are the status
      boxes that must add up to the rows, the flagged rows the pharmacist works first, and the
      one unread photo. `/prescriptions` now frames `features::PrescriptionsPage::new()`, and
      `screens/prescriptions.rs` is gone.
- [x] `features/clinical_check/`, the same four layers for the pharmacist's check of one
      prescription. Its domain holds the rule the board states in a footnote: an alert holds
      the prescription only until the call that answers it is recorded, so `can_approve` is
      false for an alert with no recorded outcome. `board::link_to` now sets a debug selector
      from the element's id, so a widget test can click the link the board named.
      `screens/prescription_check.rs` is gone and
      `/prescriptions/:prescription_id/check` frames `features::ClinicalCheckPage::new()`.
- [x] `features/shared/board/`, the toolkit every board is drawn with, moved out of the
      screens module so a feature's presentation imports `features::shared::board`. Two boundary
      tests walk the presentation sources and fail on a `screens::board` import or an import of
      another feature.
- [x] `features/patient_profile/`, the same four layers for a patient's record and the list it
      is found from: two `BLoC`s (record and directory), the refill rules (`a medicine due when
      the clinic says so is not a pharmacy refill`), and the board's seven-patient directory.
      `/patients` and `/patients/:patient_id` frame `features::PatientListPage::new()` and
      `features::PatientProfilePage::new()`, and `screens/patient_profile.rs` is gone.
- [x] The copy is English and Arabic, in `locales/`, read through rust-i18n's `t!`: the sidebar,
      the top bar and the patient feature speak both. The top bar's language button switches
      the locale, the text direction (`locale::Language`) and the font; the whole frame lays
      out inside rok-ui's `Direction`, so rows, tables and the sidebar mirror in Arabic, and
      the board draws every label through `BidiText`, which puts Arabic's letters in the right
      order on Windows. Noto Sans Arabic ships beside Outfit and Oxanium, and the boards'
      numeric columns stay right-aligned in both directions. Features translated so far:
      the frame and the patient feature; the rest move to `t!` as they are touched.
- [x] `features/licences/`, the same four layers for the folder an inspector reads: the
      board's rule that a renewal due date is day arithmetic, not text (`CivilDate` counts the
      board's 57 days from the story's date), the standing chips derived from one `Standing`
      enum, and the gap the board leads with - the licence itself is on file while the renewal
      application is not. `/licences` frames `features::LicencesPage::new()`, and
      `screens/licences_and_inspection.rs` is gone. The copy is English and Arabic.
- [ ] Reports (board already drawn in `screens/reports.rs`, to be rebuilt as `features/reports/`).

The remaining screens move across one feature at a time, each keeping the board it was drawn
from as its story data.