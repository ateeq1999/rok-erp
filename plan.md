# rok POS Pharmacy — implementation plan

This plan builds the pharmacy desktop app first, then the medicine supplier app, phase by phase. Every phase ends with something you can run and show to a pharmacist.

|                |                                                                                                                                                                              |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Stack**      | `rok-ui 0.7`, `rok-ui-hooks 0.3`, `rok-db 0.3`, GPUI `0.2.2`, PostgreSQL 16                                                                                                  |
| **Design**     | The pharmacy and supplier boards you exported (`design/`), one board per screen                                                                                              |
| **Database**   | The `rok-pos-database` modules: `core`, `catalog`, `inventory`, `customers`, `point_of_sale`, `purchasing`, `pharmacy`, `marketplace`, plus two new migrations in this plan  |
| **Story data** | Afya Pharmacy (Mwenge and Tegeta branches), Grace N. as pharmacist in charge, supplier Uzima Pharmaceuticals. The same figures as the boards, used as seed data and in tests |

---

## 0. Decisions made before Phase 1

### 0.1 Repository and names

- **Repository:** `ateeq1999/rok-pos`, which matches `rok-ui`, `rok-db` and `rok-ui-hooks`.
- **Binaries:**
  - `rok-pharmacy`: the pharmacy desktop app.
  - `rok-medicine-supplier`: the supplier desktop app, from Phase 16.
- **Crates** use the `rok-pos-` prefix, written out in full: `rok-pos-pharmacy`, not `rok-pos-pharm`.

### 0.2 Folder structure

```text
rok-pos/
├── Cargo.toml                      workspace, shared dependency versions, lints
├── rust-toolchain.toml             stable, at least 1.88 (rok-ui's minimum)
├── README.md
├── plans/
│   └── pharmacy-plan.md            this file
├── design/                         exported boards (the visual spec for each screen)
│   ├── pharmacy/                   PharmacyDashboard, PharmacyPrescriptions, … VerticalPharmacy
│   ├── supplier/                   PharmaSupplierDashboard, PharmaSupplierOrders, …
│   └── shared/                     AuthSignIn, OnboardingBusinessType, Foundations, …
├── database/
│   ├── modules/                    from rok-pos-database: <module>/module.toml + migrations/
│   │   ├── core/ catalog/ inventory/ customers/ point_of_sale/ purchasing/ marketplace/
│   │   ├── pharmacy/               0001 (existing) + 0002 clinical, stock and compliance (Phase 1)
│   │   └── medicine_supplier/      0001 (Phase 16)
│   └── seeds/
│       └── afya_pharmacy_story.sql the board's story, for demos and tests
├── crates/
│   ├── rok-pos-domain/             money, quantities, identifiers, schedules, errors (no input/output)
│   ├── rok-pos-database/           module installer, business session, rok-db models, repositories
│   ├── rok-pos-shell/              app frame: sidebar, top bar, page header, assistant button, theme
│   ├── rok-pos-hardware/           receipt and label printers, barcode scanner, cash drawer
│   ├── rok-pos-messaging/          text message and WhatsApp adapters (refills, recalls)
│   ├── rok-pos-pharmacy/           every pharmacy screen, its queries, procedures and rules
│   └── rok-pos-medicine-supplier/  every supplier screen (Phase 16)
├── apps/
│   ├── rok-pharmacy/               main.rs, routes.rs, windows (back office, dispensary till)
│   ├── rok-medicine-supplier/      main.rs, routes.rs (Phase 16)
│   └── rok-pos-server/             Axum marketplace API between pharmacies and suppliers (Phase 15)
├── tools/
│   └── migration-verifier/         applies all ups, all downs, ups again (port of the verifier)
└── .github/workflows/ci.yml
```

Inside `rok-pos-pharmacy`, there is one folder per screen. Each folder mirrors its board:

```text
crates/rok-pos-pharmacy/src/
├── lib.rs                          routes(), sidebar groups, permissions, assistant rules
├── screens/
│   ├── dashboard/                  mod.rs (screen), queries.rs, components.rs
│   ├── dispensary_till/            VerticalPharmacy
│   ├── prescriptions/              PharmacyPrescriptions
│   ├── prescription_check/         PharmacyPrescriptionCheck
│   ├── patient_profile/            PharmacyPatientProfile
│   ├── refills/                    PharmacyRefills
│   ├── medicines/                  PharmacyMedicines
│   ├── batches_and_expiry/         PharmacyStockBatches
│   ├── controlled_register/        VerticalPharmacyRegister
│   ├── order_medicines/            PharmacyOrderMedicines
│   ├── receive_delivery/           PharmacyGoodsReceipt
│   ├── recalls/                    PharmacyRecalls
│   ├── insurance_claims/           PharmacyInsuranceClaims
│   └── licences_and_inspection/    PharmacyCompliance
├── rules/                          pure functions, unit tested without a database
│   ├── first_expiry_first_out.rs   batch picking
│   ├── shelf_life.rs               minimum shelf life on delivery
│   ├── interaction_check.rs        interaction and allergy checks against rule tables
│   ├── insurance_split.rs          insurer share / patient share
│   └── controlled_register.rs      running balance and count variance
└── procedures/                     #[procedure] commands: dispense, approve_check, accept_delivery, …
```

### 0.3 How the three crates fit together

- **rok-db 0.3** is a direct dependency of `rok-pos-database`. Models, repositories and the installer live there.
- **rok-ui 0.7** uses the features `full`, `query` and `router`. The `db` feature stays off.
  - In rok-ui 0.7.0, `db` pulls in **rok-db 0.1**. Next to rok-db 0.3 that gives two copies, and models derived with 0.3 would not work with `rok_ui::db`.
  - Instead, the app registers its own `rok_db::Db` with `query::provide(cx, PharmacyDatabase(db))`. Fetchers read it from `TaskCx`.
  - This uses only the `query` feature, which does not depend on `db`.
  - Optional cleanup: publish rok-ui 0.7.1 with its `rok-db` dependency raised to `0.3`. Then `db::db_query` / `db::db_mutation` can be used too.
- **rok-ui-hooks 0.3** comes through `rok_ui::state` (re-exported as `rok_ui::state::signals`). `rok_ui::init` already runs its `tick` on GPUI's executor. It is used for app-wide stores: the session (business, branch, user, role), the dispensing basket and the till's offline queue.

```toml
# Cargo.toml (workspace)
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tools/*"]

[workspace.dependencies]
gpui = "0.2.2"
rok-ui = { version = "0.7", features = ["full", "query", "router"] }
rok-ui-hooks = "0.3"
rok-db = { version = "0.3", features = ["chrono", "uuid", "json", "serde", "testing"] }
sqlx = { version = "0.8", default-features = false, features = ["rust_decimal"] }
rust_decimal = "1"
uuid = { version = "1", features = ["v7"] }
chrono = "0.4"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
thiserror = "2"
serde = { version = "1", features = ["derive"] }
toml = "0.8"
```

### 0.4 Where the database runs

rok-db is PostgreSQL only, so each pharmacy runs one PostgreSQL 16 server on its back-office computer. Tills on the same network connect to it.

- **Development:** a Docker Compose file starts PostgreSQL 16.
- **Customer machines:** the Windows installer bundles PostgreSQL as a service, set up by the installer (Phase 17).
- **Branches** (Mwenge and Tegeta) each have their own server at first. Phase 14 adds sync to the shared server.

Code blocks in this plan are sketches that show the shape. Check exact signatures against the crate docs while building.

### 0.5 Business isolation on every query

The schema already isolates each business with row level security, which reads `app.organization_id`. rok-db adds its own tenant filter with `#[rok(tenant)]`. Use both:

- the ORM filter keeps everyday queries correct;
- row level security catches raw SQL and mistakes.

```rust
// crates/rok-pos-database/src/business_session.rs
pub async fn in_business<T, F>(database: &Db, session: &BusinessSession, work: F) -> rok_db::Result<T>
where
    F: for<'transaction> FnOnce(&'transaction mut Tx) -> BoxFuture<'transaction, rok_db::Result<T>> + Send,
    T: Send,
{
    let session = session.clone();
    rok_db::tenant::with_tenant(session.organization_id, async move {
        database.transaction(|transaction| Box::pin(async move {
            rok_db::raw("select set_config('app.organization_id', ?, true), set_config('app.user_id', ?, true)")
                .bind(session.organization_id.to_string())
                .bind(session.user_id.to_string())
                .execute(&mut *transaction).await?;
            work(transaction).await
        })).await
    }).await
}
```

The app connects as a role that does not own the tables and has no `bypassrls`. Migrations run as the owner role.

### 0.7 Money

- Columns are `numeric(18,2)`. In Rust they are `rust_decimal::Decimal`, through sqlx's `rust_decimal` feature.
- Wrap it in a `Money` newtype and register it with `rok_db::impl_value!(Money)`.
- This is checked in the Phase 0 spike. If `impl_value!` can't carry `Decimal`, add a `decimal` feature to rok-db (your crate) instead of storing floats.

### 0.7 Medical content

- Interaction and allergy checks run against **rule tables** (`pharmacy.interaction_rules`). They are loaded from a reference source the pharmacy has the right to use, and reviewed by a pharmacist.
- The app never invents a clinical rule, and the assistant never makes a clinical decision.
- Every alert names its source and asks the pharmacist to decide.

---

## Phase 0 — Workspace, shell and spikes (1 week)

**Goal:** an empty `rok-pharmacy` window that already looks like the boards. The four risky integrations are proven.

**Screens:** the frame of `design/pharmacy/PharmacyDashboard` (sidebar, top bar, empty main area).

**Steps**

1. Create the workspace from §0.2 with `rust-toolchain.toml`, workspace lints (`clippy::pedantic`, `missing_docs` on library crates), `rustfmt.toml` and CI (fmt, clippy, tests with a PostgreSQL service).
2. `rok-pos-shell`:
   - [x] `theme.rs`: start from `ThemePreset::Rok`, with the pharmacy accent `#0F766E` (tint `#F0FDFA`, text on tint `#115E59`) as a business-type accent.
   - [x] `fonts.rs`: register Oxanium (numbers, codes) and Outfit (text) with `rok_ui::fonts::register_font_files`.
   - [x] `app_frame.rs`: `AppRoot` → `Sidebar` (groups and badges from `PharmacySidebar`) + `AppBar` top bar + router outlet.
   - [x] `money_text.rs`, `batch_code_text.rs`: Oxanium, thousands separators, TZS without decimals.
3. `apps/rok-pharmacy/src/main.rs`:
   - [x] `Application::new().with_assets(rok_ui::Assets)`, `rok_ui::init(cx)`, register fonts, open the back-office window.
   - [x] `Router` with all pharmacy routes pointing at placeholder pages: `/`, `/prescriptions`, `/prescriptions/:prescription_id/check`, `/patients/:patient_id`, `/refills`, `/medicines`, `/batches`, `/controlled-register`, `/order`, `/receive/:order_id`, `/recalls/:recall_id`, `/claims`, `/licences`, `/till`.
4. Spikes, each a small test or example:
   - [x] `Money` round trip with `rok_db::impl_value!` (§0.7).
   - [x] `in_business` with `with_tenant` and `set_config`, checked as the non-owner role: another business's rows are invisible, even through `rok_db::raw`.
   - [x] `query::provide(cx, PharmacyDatabase(db))` and a `use_query` that lists rows on screen.
   - [x] Print one dosage label and one 80 mm receipt (ESC/POS) from `rok-pos-hardware`.

**Done when:** the window opens with the pharmacy sidebar, every sidebar link changes the route, and the four spikes pass in CI.

---

## Phase 1 — Database and module installer (1–2 weeks)

**Goal:** one command installs every module the pharmacy needs on a fresh PostgreSQL, safely and repeatably.

**Steps**

1. Copy `rok-pos-database/modules` into `database/modules`.
2. The module installer in `rok-pos-database/src/module_installer.rs` is our own, not `Db::migrate`. sqlx's migrator keeps one version list per database, and module migrations reuse numbers such as `0001`.
   - [x] Read every `module.toml` (serde + `toml`), sort by `depends_on`, refuse cycles.
   - [x] Run each pending `NNNN_*.up.sql` in its own transaction with `rok_db::raw`.
   - [x] Record it in `core.applied_migrations` with a SHA-256 checksum.
   - [x] Refuse to start if a shipped migration's checksum changed.
   - [x] Upsert `[[permissions]]` into `core.permissions`.
   - [x] Write progress to `core.module_jobs.steps`. The install screen (Phase 2) shows it.
3. New migration `database/modules/pharmacy/migrations/0002_add_clinical_stock_and_compliance.up.sql`, with its `.down.sql`. Same conventions as every module: UUID version 7 keys, `organization_id`, timestamps, `row_version`, `core.prepare_table(..)`.

   | Table                                | Purpose and key columns                                                                                                                                                                                                                                                                                                                                                                                                                          |
   | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
   | `pharmacy.medicine_details`          | One row per medicine product: `product_id` (unique, → `catalog.products`), `generic_name`, `strength_text`, `dosage_form`, `pack_size_text`, `medicine_schedule` (`general_sale`, `pharmacy_medicine`, `prescription_only`, `controlled`), `storage_condition` (`room_temperature`, `refrigerated_two_to_eight`), `regulator_registration_number`, `minimum_shelf_life_months_on_delivery`, `label_warnings text[]`, `is_on_insurance_formulary` |
   | `pharmacy.medicine_substitutes`      | `product_id`, `substitute_product_id`, `substitution_kind` (`same_generic`, `therapeutic`)                                                                                                                                                                                                                                                                                                                                                       |
   | `pharmacy.patient_clinical_profiles` | `customer_id` (unique), `date_of_birth`, `sex`, `allergies text[]`, `conditions_from_prescriptions text[]`, `insurance_provider_id`, `insurance_member_number`, `reminder_consent`, `reminder_consent_given_at`                                                                                                                                                                                                                                  |
   | `pharmacy.clinical_notes`            | `customer_id`, `prescription_id`, `note_text`, `written_by_user_id`. Readable only with `pharmacy.clinical_notes.view`                                                                                                                                                                                                                                                                                                                           |
   | `pharmacy.interaction_rules`         | `first_generic_name`, `second_generic_name`, `severity` (`information`, `caution`, `serious`), `message_text`, `source_reference`, `reviewed_by_user_id`, `reviewed_on`                                                                                                                                                                                                                                                                          |
   | `pharmacy.prescription_checks`       | `prescription_id`, `check_key` (`identity`, `allergy`, `interaction`, `duplicate_therapy`, `dose_range`, `insurance_cover`), `result` (`passed`, `warning`, `failed`), `details`, `checked_by_user_id`                                                                                                                                                                                                                                           |
   | `pharmacy.prescriber_contacts`       | `prescription_id`, `contacted_at`, `contact_method`, `outcome` (`prescriber_agreed`, `prescription_changed`, `no_answer`), `note_text`, `recorded_by_user_id`                                                                                                                                                                                                                                                                                    |
   | `pharmacy.dispensing_labels`         | `prescription_item_id`, `label_text`, `printed_at`, `printed_by_user_id`, `reprint_count`                                                                                                                                                                                                                                                                                                                                                        |
   | `pharmacy.refill_schedules`          | `customer_id`, `product_id`, `days_of_supply`, `last_filled_on`, `next_due_on`, `reminder_status` (`not_due`, `scheduled`, `sent`, `no_consent`, `collected`)                                                                                                                                                                                                                                                                                    |
   | `pharmacy.temperature_logs`          | `branch_id`, `storage_unit_name` (Fridge 1), `recorded_at`, `temperature_celsius numeric(5,2)`, `is_in_range`, `recorded_by_user_id`                                                                                                                                                                                                                                                                                                             |
   | `pharmacy.recall_notices`            | `supplier_name`, `marketplace_recall_reference`, `product_id`, `batch_number`, `reason_text`, `recall_class`, `issued_at`, `instructions_text`, `status` (`open`, `quarantined`, `returned`, `closed`)                                                                                                                                                                                                                                           |
   | `pharmacy.recall_actions`            | `recall_notice_id`, `action_type` (`quarantined`, `patient_contacted`, `returned_to_supplier`, `credit_received`), `customer_id`, `quantity`, `done_at`, `done_by_user_id`, `note_text`                                                                                                                                                                                                                                                          |
   | `pharmacy.licence_documents`         | `branch_id`, `licence_type` (`premises`, `pharmacist_registration`, `business`, `controlled_permit`, `fire_certificate`, `fridge_calibration`), `holder_name`, `licence_number`, `issued_on`, `expires_on`, `attachment_id`                                                                                                                                                                                                                      |
   | `pharmacy.receipt_quality_checks`    | `goods_receipt_line_id` (→ `purchasing.goods_receipt_lines`), `check_key` (`expiry`, `packaging`, `cold_chain`, `quantity`), `result`, `minimum_temperature_celsius`, `maximum_temperature_celsius`, `note_text`                                                                                                                                                                                                                                 |
   | `pharmacy.insurance_claim_batches`   | `insurance_provider_id`, `period_start_on`, `period_end_on`, `status` (`open`, `submitted`, `queried`, `paid`), `submitted_at`, `claimed_amount`, `paid_amount`, `deducted_amount`, plus a `claim_batch_id` column added to `pharmacy.insurance_claims`                                                                                                                                                                                          |

   Add the new permissions to `pharmacy/module.toml`:
   - `pharmacy.clinical_check.approve`
   - `pharmacy.clinical_notes.view`
   - `pharmacy.recalls.manage`
   - `pharmacy.licences.manage`
   - `pharmacy.temperature_logs.record`
   - `pharmacy.interaction_rules.manage`

4. rok-db models in `rok-pos-database/src/models/`, one file per table group. Statuses become `DbEnum`s stored as text, matching the `check` constraints.

   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, DbEnum)]
   pub enum MedicineSchedule { GeneralSale, PharmacyMedicine, PrescriptionOnly, Controlled }

   #[derive(Debug, Clone, Model)]
   #[rok(table = "pharmacy.medicine_details", timestamps, soft_delete)]
   pub struct MedicineDetails {
       #[rok(primary_key, generated)] pub id: Uuid,
       #[rok(tenant)] pub organization_id: Uuid,
       #[rok(belongs_to = Product)] pub product_id: Uuid,
       pub generic_name: String,
       pub strength_text: String,
       pub dosage_form: String,
       pub medicine_schedule: MedicineSchedule,
       pub storage_condition: StorageCondition,
       pub minimum_shelf_life_months_on_delivery: i32,
       pub label_warnings: Vec<String>,
       #[rok(version)] pub row_version: i64,
       pub created_at: DateTime<Utc>,
       pub updated_at: DateTime<Utc>,
       pub deleted_at: Option<DateTime<Utc>>,
   }
   ```

5. `database/seeds/afya_pharmacy_story.sql`: the board's story. It includes:
   - both branches and the staff;
   - medicines and batches (AMX-2409, AMS-2404, TRM-2411 …);
   - prescriptions RX-2210 to RX-2219;
   - patients Ali Hassan and Mzee Salim R.;
   - the controlled register balance of 80;
   - order UZ-7781 and recall RC-0047;
   - insurance batches.
6. `tools/migration-verifier`: the Rust port of the verifier (all ups, all downs, ups again). Run it in CI.

**Tests:** installer runs twice with no changes; checksum tampering is refused; row level security checks for every new table use `#[rok_db::test]` as the non-owner role.

**Done when:** `rok-pharmacy --install` on an empty database installs the eight modules in order and loads the story seed, and CI's verifier passes.

---

## Phase 2 — Onboarding a pharmacy (1 week)

**Goal:** a new pharmacy owner goes from a fresh install to a ready back office in under ten minutes.

**Screens:**

- `design/shared/AuthCreateAccount`
- `design/shared/OnboardingBusinessType` (pharmacy chosen)
- `design/shared/OnboardingBusinessSetup`
- the install progress from `design/shared/AppsInstall`

**Steps**

1. Owner sign-up: name, phone or email, password, 6-digit verification (`InputOtp`).
2. Business type "Pharmacy" turns on these modules: catalog, inventory, customers, point_of_sale, purchasing and pharmacy.
3. Pharmacy setup, a `Questionnaire` or stepped `Field` form:
   - business name and branches;
   - pharmacist in charge (name, registration number);
   - premises licence number and expiry;
   - insurers accepted;
   - the receipt and label printer.

   These answers write `core.branches`, `pharmacy.licence_documents`, `pharmacy.insurance_providers` and `core.settings`.

4. Activation per business, from each manifest:
   - roles: owner, pharmacist in charge, pharmacist, pharmaceutical technologist, dispenser, stock clerk, accountant;
   - number sequences: RX-, S-, PO-, CN-;
   - default assistant rules.
5. "Load demo pharmacy" option, which loads the Afya story so the owner can explore first.

**Done when:** a fresh install ends on the pharmacy dashboard frame with the owner signed in and the licence expiry stored.

---

## Phase 3 — Sign-in, staff and roles (1 week)

**Goal:** each person sees only what their role allows. Pharmacist actions need the pharmacist's own PIN.

**Screens:**

- `AuthSignIn`, `AuthPinLock`, `AuthForgotPassword`, `AuthVerifyCode`
- `AuthStaffInvite`, `OfficeStaff`, `OfficeRoles`, `AccountProfile`

**Steps**

1. Password sign-in. Hash passwords and PINs with a memory-hard function (argon2) and limit attempts.
2. PIN lock and switch user on a shared till. The session store (`rok_ui::state` store) holds the business, branch, user and role, and updates the sidebar.
3. Permission checks in one place: `require_permission(&session, "pharmacy.clinical_check.approve")`. They are used by every procedure and by the sidebar to hide items.
4. Pharmacist sign-off component, `PharmacistSignOff`:
   - asks for a pharmacist's PIN;
   - returns the signing user;
   - writes `core.audit_events`.

   It is reused by dispensing, controlled items, recalls and disposal.

5. Staff invite with role and branch. Store the pharmacist registration number on the user's `custom_fields`.

**Done when:** a dispenser can sell general sale medicines but can't approve a prescription; Grace N.'s PIN unlocks the approval.

---

## Phase 4 — Medicine catalogue (1–2 weeks)

**Goal:** the pharmacy's medicines are loaded with everything the till and the rules need.

**Screen:** `design/pharmacy/PharmacyMedicines`

**Steps**

1. `DataTable` with these columns:
   - generic name, brand/strength/form, pack;
   - schedule chip, storage (room or 2–8 °C), regulator registration;
   - stock on hand, nearest expiry, price, formulary.

   Filter tabs by schedule; filter by storage.

2. Editor `Sheet` on the right. It writes `catalog.products` and `pharmacy.medicine_details` in one `in_business` transaction. It covers:
   - generic name, strength, form, schedule;
   - the cold-chain `Switch`, minimum shelf life, reorder point;
   - substitutes, insurance codes, label warnings.
3. Import a supplier price list (CSV) with a preview of errors before saving.
4. Barcodes per pack (`catalog.product_barcodes`, with `units_per_scan` for boxes).
5. Queries: `query_key!["medicines", filter]`. The `save_medicine` procedure invalidates `["medicines"]`.

**Tests:** a controlled medicine without a schedule can't be saved; import rejects duplicate barcodes.

**Done when:** the 12 board medicines exist with their schedules, and the catalogue matches the board.

---

## Phase 5 — Stock by batch and expiry (2 weeks)

**Goal:** every unit on the shelf belongs to a batch with an expiry date. The system always offers the earliest-expiring batch first.

**Screen:** `design/pharmacy/PharmacyStockBatches`

**Steps**

1. Opening stock entry per batch: batch number, expiry, quantity, cost. This writes `inventory.stock_batches`, `inventory.stock_levels` and an `opening_balance` movement.
2. `rules/first_expiry_first_out.rs`, a pure function:
   - picks batches for a quantity;
   - skips expired, quarantined and recalled batches;
   - returns the pick plus warnings.

   Unit-test it heavily.

3. Expiry views:
   - tiles for expired on shelf, expiring in 30 days, expiring in 90 days;
   - a batches table across both branches with status chips;
   - "Sell first" pin for the till.
4. Quarantine, return to supplier and disposal. Disposal needs a pharmacist witness through `PharmacistSignOff`. Every action is a stock movement, never an edit.
5. Fridge temperature log: entry form, the last 24 hours as a `Chart` (line) with the 2–8 °C band, and an out-of-range alert.
6. Block expired and quarantined batches at the till, using the same rule function.

**Done when:** the board's numbers come out of real queries: 2 expired batches, 5 expiring in 30 days worth 186,400, and 14 in 90 days worth 612,900.

---

## Phase 6 — Dispensary till (3 weeks)

**Goal:** sell over-the-counter medicines and dispense prescriptions quickly, with the right batch, split payment and labels.

**Screen:** `design/pharmacy/VerticalPharmacy` (its own full-screen window), plus `SellPayment` and `SellPaymentComplete` from the shared boards.

**Steps**

1. Till window: register bar, product search (`Command` with barcode scanner input), basket and payment panel. The basket is a rok-ui-hooks store, so it survives screen changes.
2. Each line takes its batch from the first-expiry rule and shows batch and expiry, as on the board.
3. Schedule rules at the till:
   - **General sale:** any cashier can sell it.
   - **Pharmacy medicine:** needs a pharmacist on duty.
   - **Prescription-only:** needs a prescription (Phase 7).
   - **Controlled:** needs a prescription and the pharmacist's PIN, and writes the register (Phase 8).
4. Payment:
   - cash, mobile money, card;
   - the insurance split (`rules/insurance_split.rs`). RX-2210: 13,700 total, insurer 9,590, patient 4,110;
   - medicines are VAT exempt.
5. Shifts: open with a float, cash in and out, close with a blind count and the end-of-day report.
6. Receipt (80 mm) and dosage labels: patient, medicine, directions, date, pharmacist initials, label warnings. Each label is written to `pharmacy.dispensing_labels`.
7. Returns: only unopened, non-controlled items, with pharmacist approval.

**Tests:**

- the RX-2210 sale reproduces the board's totals to the shilling;
- an expired batch can't be sold;
- a controlled line without a PIN is refused.

**Done when:** a dispenser serves 50 mixed sales in a test script and the end-of-day report matches the sales.

---

## Phase 7 — Prescriptions and the pharmacist's check (2–3 weeks)

**Goal:** every prescription goes through a recorded check before anything is dispensed.

**Screens:** `PharmacyPrescriptions`, `PharmacyPrescriptionCheck`

**Steps**

1. Prescription queue:
   - status tiles: new, needs check, waiting for prescriber, ready, dispensed today;
   - sources: paper scan, WhatsApp photo, e-prescription;
   - the scan is stored in `core.attachments`.
2. Typing a prescription: patient lookup or quick add, prescriber, items mapped to catalogue medicines, directions.
3. `rules/interaction_check.rs`:
   - compares the new items and the patient's current medicines against `pharmacy.interaction_rules` and the patient's allergies;
   - results go to `pharmacy.prescription_checks`.
   - RX-2214 shows the warfarin and metronidazole caution with its source.
4. Prescriber call: record the outcome in `pharmacy.prescriber_contacts`, add a clinical note, add counselling points.
5. Label preview. "Approve for dispensing" needs `pharmacy.clinical_check.approve` and the pharmacist's PIN. The prescription then appears on the till as ready.
6. Interaction rules screen, for pharmacists: list and review the rule table. The team loads rules from the reference source they're allowed to use (§0.7).

**Done when:** RX-2214 can only be dispensed after the check is recorded and approved, and the audit log shows who approved it and why.

---

## Phase 8 — Controlled medicines register (1 week)

**Goal:** a register an inspector can trust: append-only, signed, balanced.

**Screen:** `design/pharmacy/VerticalPharmacyRegister`

**Steps**

1. Every controlled movement (received, dispensed, destroyed, returned, count) writes `pharmacy.controlled_substance_entries` in the same transaction as the stock movement.
2. Running balance and count variance come from `rules/controlled_register.rs`. A mismatch blocks further controlled sales until a pharmacist records an explanation.
3. Daily count screen. Corrections are new signed entries, never edits; a test checks that the row level policy and the app refuse updates.
4. Export the register as PDF and CSV for inspection.

**Done when:** the Tramadol register shows received 100, dispensed 20 and 10, balance 80, the 08:15 count, and the export matches.

---

## Phase 9 — Patients and refills (1–2 weeks)

**Goal:** chronic patients come back on time, and the pharmacist sees their history in one place.

**Screens:** `PharmacyPatientProfile`, `PharmacyRefills`

**Steps**

1. Patient record:
   - identity with masked phone, insurance, allergies;
   - conditions from prescriptions, current medicines, dispensing history;
   - clinical notes, visible to pharmacists only;
   - reminder consent `Switch`.
2. Refill schedules are created when a chronic medicine is dispensed: days of supply → next due date.
3. Refills due list for the next 7 days, with reminder status and "Prepare in advance".
4. `rok-pos-messaging`:
   - a text message adapter trait plus one provider;
   - Kiswahili and English templates;
   - only patients with consent;
   - every message logged.

**Done when:** the 9 refills on the board come from the schedule. A reminder to Mzee Salim R. is sent only because consent is on.

---

## Phase 10 — Insurance claims (2 weeks)

**Goal:** every insured sale becomes a claim. Queried claims are fixed and resubmitted without paperwork.

**Screen:** `design/pharmacy/PharmacyInsuranceClaims`

**Steps**

1. A claim is created at the till for each insured sale (member number, items, amounts).
2. Monthly batches per insurer:
   - **September, national insurer:** 214 claims, 4,862,300.
   - **October:** builds as sales happen. RX-2210's 9,590 goes into it.
3. Queries list with reason and fix action. Resubmit the fixed claims.
4. Export formats differ by insurer, so each one is an adapter behind one trait. Start with a CSV/Excel export, then each insurer's required format.
5. Payment matching: record payments and deductions per batch.

**Done when:** the 11 queried claims show their reasons, and fixing one moves it to "Fixed, ready to resubmit".

---

## Phase 11 — Ordering medicines and receiving deliveries (2 weeks)

**Goal:** order from suppliers and accept only stock that meets the pharmacy's rules.

**Screens:** `PharmacyOrderMedicines`, `PharmacyGoodsReceipt`

**Steps**

1. Order screen, with suppliers stored locally first (Phase 15 connects the marketplace):
   - suggested quantities from reorder points;
   - the cart;
   - approval over 300,000 by the pharmacist in charge;
   - controlled lines need a permit on file.
2. Purchase order UZ-7781 (518,700) is stored in `purchasing.purchase_orders`.
3. Receiving per line: scan the batch, read the expiry, count. `rules/shelf_life.rs` rejects any batch under the medicine's minimum shelf life:
   - Cetirizine expires 03/2027, under 12 months → rejected, credit note CN-0932 for 30,000.
   - accepted value 488,700.
4. Cold-chain check: enter or import the logger's minimum and maximum. In range (2–8 °C) is accepted; out of range goes to quarantine and needs a pharmacist decision.
5. "Accept into stock", with the pharmacist's PIN:
   - creates batches and movements for accepted lines only;
   - updates average cost;
   - matches the supplier invoice.

**Done when:** receiving UZ-7781 reproduces the board: 5 lines accepted, 1 rejected, the credit note, and stock up by exactly the accepted batches.

---

## Phase 12 — Recalls (1 week)

**Goal:** when a batch is recalled, the pharmacy stops it, finds every unit and every patient, and returns it, with proof.

**Screen:** `design/pharmacy/PharmacyRecalls`

**Steps**

1. Entering a recall notice is manual at first (Phase 15 receives it from the supplier automatically). Entering it immediately:
   - blocks the batch at every till;
   - quarantines the stock on hand;
   - finds the sales of that batch.
2. Patients to contact: they come from the sales and prescription links (RC-0047: 6 bottles sold to 5 patients). Contact them by text message (Phase 9), or call and record the outcome.
3. Return to supplier: the quantity and the expected credit (14 bottles, 58,800).
4. Checklist and sign-off by the pharmacist in charge. Close the recall when every step is done.

**Done when:** creating RC-0047 blocks AMS-2404 within a second at both branches, and the patient list matches the board.

---

## Phase 13 — Dashboard, licences and reports (1–2 weeks)

**Goal:** the owner and pharmacist in charge see the day and stay inspection-ready.

**Screens:** `PharmacyDashboard`, `PharmacyCompliance`, `OfficeReports` (pharmacy filters)

**Steps**

1. Dashboard:
   - tiles for today's sales (with insurance share), prescriptions dispensed and waiting, expiring value, queried claims;
   - a "Needs you now" list: recall, interaction checks, deliveries, licence renewal, controlled count;
   - sales by hour (`Chart`), payment split, branch comparison.
2. Licences and inspection:
   - licence cards with expiry warnings (premises licence due in 57 days);
   - readiness checklist computed from the data (register up to date, fridge logs complete, expired stock segregated, staff certificates on file);
   - document uploads;
   - "Export inspection pack (PDF)".
3. Reports:
   - sales by medicine and schedule;
   - margin;
   - expiry losses;
   - claims aging;
   - controlled movements.

**Done when:** the dashboard's tiles and the readiness checklist are computed from the story data and match the boards.

---

## Phase 14 — Msaidizi in the pharmacy (1–2 weeks)

**Goal:** the assistant saves time without making clinical decisions.

**Steps**

1. The floating assistant button (`PosAssistant` board) in the app frame, with page rules from `ai_assistant.page_rules`. Pharmacy defaults:
   - **Refills:** "9 refills due this week".
   - **Batches:** "5 batches expire within 30 days, worth 186,400".
   - **Recalls:** "Recall RC-0047: 2 patients not yet reached".
   - **Order:** "Insulin cover is 4 vials, below reorder point".
2. Actions only open pages or prepare drafts (a draft order, a reminder message). The person confirms.
3. Hard rule: on clinical screens the assistant may summarise the record and the recorded checks. It never suggests doses or overrides an interaction alert. This is enforced in the prompt and by giving it no clinical tools.

**Done when:** each default rule fires on the story data, and no assistant action changes data without confirmation.

---

## Phase 15 — Shared server and marketplace connection (3 weeks)

**Goal:** pharmacies and suppliers exchange orders, deliveries and recalls through one server.

**Steps**

1. `apps/rok-pos-server`, an Axum app:
   - business sign-in with tokens;
   - marketplace endpoints for listings, orders, order status, shipments, recalls and licence documents;
   - the same database modules, with `marketplace` row level security (buyer and vendor policies already in the schema).
2. Branch sync: Mwenge and Tegeta push changes from `core.change_log` and pull the others'. Conflicts go to `core.sync_conflicts` with a resolve screen.
3. Pharmacy side:
   - the order screen lists marketplace suppliers;
   - order status and shipment tracking come from the server;
   - recall notices arrive automatically and trigger Phase 12's blocking;
   - licence documents are shared with the supplier for verification.
4. Offline: the pharmacy keeps working when the server is unreachable. Orders queue and send later.

**Done when:** placing UZ-7781 in the pharmacy app shows it as a new order to the supplier, and a recall issued by the supplier blocks the batch at Afya without manual entry.

---

## Phase 16 — Medicine supplier app (4 weeks)

**Goal:** Uzima Pharmaceuticals runs its sales to pharmacies in `rok-medicine-supplier`.

**Screens (`design/supplier/`):**

- `PharmaSupplierDashboard`, `PharmaSupplierOrders`, `PharmaSupplierCatalogue`
- `PharmaSupplierCustomers`, `PharmaSupplierDeliveries`, `PharmaSupplierRecalls`
- the `PharmaSupplierNav` top navigation

**Database:** new module `database/modules/medicine_supplier/migrations/0001_create_supplier_operations.up.sql`.

| Table                                        | Purpose                                                                                          |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `medicine_supplier.customer_licence_checks`  | Each pharmacy customer's licence, pharmacist and permit documents with status, checker and dates |
| `medicine_supplier.controlled_sales_entries` | Supplier's controlled register: customer, permit number, product, batch, quantity, balance       |
| `medicine_supplier.batch_allocations`        | Which batch each order line was picked from, with the shelf-life rule result                     |
| `medicine_supplier.recall_distributions`     | Per recall and customer: quantity sent, quarantine confirmed, quantity returned                  |
| `medicine_supplier.cold_chain_readings`      | Per shipment and cold box: reading time and temperature                                          |

**Steps**

1. Today: orders value, deliveries, cold room temperature, receivables, "Needs you".
2. Orders:
   - accept, change or substitute lines;
   - first-expiry batch allocation, using the same `first_expiry_first_out` rule crate, that respects each customer's minimum shelf life;
   - controlled lines check the customer's permit;
   - pick lists.
3. Catalogue and batches: price tiers, batch locations, quarantine, the regulator registration placeholder.
4. Customers and licences: verify new pharmacies, pause prescription-only sales when a licence expires, credit limits and terms.
5. Cold-chain deliveries: van routes, temperature readings per cold box, proof of delivery, returns on the van.
6. Recalls and returns:
   - issue a recall: the distribution list is computed from the batch allocations, and notices go out through the server;
   - track confirmations and returns;
   - credit notes.

**Done when:** recall RC-0047 issued in the supplier app lists the 6 customers and 412 bottles, and returns update as pharmacies send bottles back.

---

## Phase 17 — Packaging, pilot and release (2–3 weeks, then ongoing)

**Steps**

1. Windows installer:
   - the app, a bundled PostgreSQL service, the first-run installer from Phase 1;
   - nightly encrypted backups and a restore check.
2. Auto-update with staged rollout. Module upgrades run through the same installer and checksums.
3. Security review:
   - row level security tests for every table;
   - PIN and password limits;
   - secrets kept out of the database;
   - `cargo deny`.
4. A pharmacist reviews:
   - the interaction rule source, label text, counselling text and controlled register export;
   - the insurance and regulator requirements in Tanzania for each export format.
5. Pilot: one pharmacy with two branches and one supplier, for four weeks with weekly fixes, then release.

---

## Order of work at a glance

| Phase | Builds                                    | Main board(s)                                                | About     |
| ----- | ----------------------------------------- | ------------------------------------------------------------ | --------- |
| 0     | Workspace, shell, spikes                  | PharmacyDashboard frame                                      | 1 week    |
| 1     | Database, installer, pharmacy 0002, seeds | —                                                            | 1–2 weeks |
| 2     | Onboarding                                | OnboardingBusinessType, OnboardingBusinessSetup, AppsInstall | 1 week    |
| 3     | Sign-in, staff, roles, pharmacist PIN     | AuthSignIn, AuthPinLock, OfficeRoles                         | 1 week    |
| 4     | Medicine catalogue                        | PharmacyMedicines                                            | 1–2 weeks |
| 5     | Batches, expiry, fridge log               | PharmacyStockBatches                                         | 2 weeks   |
| 6     | Dispensary till                           | VerticalPharmacy                                             | 3 weeks   |
| 7     | Prescriptions and the pharmacist's check  | PharmacyPrescriptions, PharmacyPrescriptionCheck             | 2–3 weeks |
| 8     | Controlled register                       | VerticalPharmacyRegister                                     | 1 week    |
| 9     | Patients and refills                      | PharmacyPatientProfile, PharmacyRefills                      | 1–2 weeks |
| 10    | Insurance claims                          | PharmacyInsuranceClaims                                      | 2 weeks   |
| 11    | Ordering and receiving                    | PharmacyOrderMedicines, PharmacyGoodsReceipt                 | 2 weeks   |
| 12    | Recalls                                   | PharmacyRecalls                                              | 1 week    |
| 13    | Dashboard, licences, reports              | PharmacyDashboard, PharmacyCompliance                        | 1–2 weeks |
| 14    | Msaidizi in the pharmacy                  | PosAssistant                                                 | 1–2 weeks |
| 15    | Shared server, marketplace, branch sync   | —                                                            | 3 weeks   |
| 16    | Medicine supplier app                     | PharmaSupplier*                                              | 4 weeks   |
| 17    | Packaging, pilot, release                 | —                                                            | 2–3 weeks |

**First usable release for a single pharmacy: Phases 0–8**, about 13–16 weeks for one developer. That covers selling, dispensing with checks, stock by batch, and the controlled register. Everything after that adds to a pharmacy that is already trading.

## Rules for every phase

- **Build from the board.** Before a screen is done, compare it side by side with its exported board: layout, wording, numbers.
- **Seed and test with the story.** The figures on the boards are the test fixtures. If a query's result differs from the board, either the query or the board is wrong; fix one of them on purpose.
- **Rules are pure functions.** First expiry first out, shelf life, interactions, insurance split and the controlled balance live in `rules/` with unit tests, and are shared by the pharmacy and supplier apps.
- **Every money, stock or clinical change** happens in one `in_business` transaction, writes a movement or ledger row, and leaves an audit event.
- **Names are written out in full** in code, tables, files and components.
