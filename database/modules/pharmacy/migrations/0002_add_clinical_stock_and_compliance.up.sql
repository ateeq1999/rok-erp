-- Module: pharmacy
-- Migration 0002: clinical records and checks, stock and expiry helpers, and
-- the compliance tables an inspection asks for.
--
-- Conventions are the same as every other module: UUID version 7 keys,
-- organization_id on every business table, created_at/updated_at/deleted_at,
-- row_version, and core.prepare_table(..) after each table for the triggers
-- and the row level security policy.

-- One row per medicine product: what the till, the labels and the rules need
-- beyond the catalogue row.
create table pharmacy.medicine_details (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  product_id uuid not null references catalog.products (id) on delete cascade,
  generic_name text not null,
  strength_text text not null,
  dosage_form text not null,
  pack_size_text text,
  medicine_schedule text not null default 'prescription_only'
    check (medicine_schedule in ('general_sale', 'pharmacy_medicine', 'prescription_only', 'controlled')),
  storage_condition text not null default 'room_temperature'
    check (storage_condition in ('room_temperature', 'refrigerated_two_to_eight')),
  regulator_registration_number text,
  minimum_shelf_life_months_on_delivery integer not null default 12 check (minimum_shelf_life_months_on_delivery >= 0),
  label_warnings text[] not null default '{}',
  is_on_insurance_formulary boolean not null default false,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  unique (product_id)
);
select core.prepare_table('pharmacy.medicine_details');
create index medicine_details_generic_name_index on pharmacy.medicine_details (organization_id, generic_name);
create index medicine_details_schedule_index on pharmacy.medicine_details (organization_id, medicine_schedule);

-- A medicine that may be dispensed in place of another, decided by the pharmacist.
create table pharmacy.medicine_substitutes (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  product_id uuid not null references catalog.products (id) on delete cascade,
  substitute_product_id uuid not null references catalog.products (id) on delete cascade,
  substitution_kind text not null check (substitution_kind in ('same_generic', 'therapeutic')),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  check (product_id <> substitute_product_id),
  unique (product_id, substitute_product_id)
);
select core.prepare_table('pharmacy.medicine_substitutes');

-- What the pharmacist needs to know about a patient before dispensing.
create table pharmacy.patient_clinical_profiles (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  customer_id uuid not null references customers.customers (id) on delete cascade,
  date_of_birth date,
  sex text check (sex in ('female', 'male', 'other', 'unknown')),
  allergies text[] not null default '{}',
  conditions_from_prescriptions text[] not null default '{}',
  insurance_provider_id uuid references pharmacy.insurance_providers (id) on delete set null,
  insurance_member_number text,
  reminder_consent boolean not null default false,
  reminder_consent_given_at timestamptz,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  unique (customer_id),
  check (not reminder_consent or reminder_consent_given_at is not null)
);
select core.prepare_table('pharmacy.patient_clinical_profiles');

-- A note about a patient's care. Readable only with pharmacy.clinical_notes.view.
create table pharmacy.clinical_notes (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  customer_id uuid not null references customers.customers (id) on delete cascade,
  prescription_id uuid references pharmacy.prescriptions (id) on delete set null,
  note_text text not null,
  written_by_user_id uuid not null references core.users (id),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1
);
select core.prepare_table('pharmacy.clinical_notes');
create index clinical_notes_customer_index on pharmacy.clinical_notes (customer_id, created_at);

-- The interaction rule table. Rows are loaded from a reference source the
-- pharmacy has the right to use (plan section 0.7) and reviewed by a
-- pharmacist; the app never invents a rule.
create table pharmacy.interaction_rules (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  first_generic_name text not null,
  second_generic_name text not null,
  severity text not null default 'caution' check (severity in ('information', 'caution', 'serious')),
  message_text text not null,
  source_reference text not null,
  reviewed_by_user_id uuid references core.users (id),
  reviewed_on date,
  is_active boolean not null default true,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1
);
select core.prepare_table('pharmacy.interaction_rules');
-- The pair is read in both orders, so one index covers the lookup either way.
create index interaction_rules_pair_index on pharmacy.interaction_rules (lower(first_generic_name), lower(second_generic_name));

-- Every check a pharmacist works through before approving a prescription.
create table pharmacy.prescription_checks (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  prescription_id uuid not null references pharmacy.prescriptions (id) on delete cascade,
  check_key text not null
    check (check_key in ('identity', 'allergy', 'interaction', 'duplicate_therapy', 'dose_range', 'insurance_cover')),
  result text not null check (result in ('passed', 'warning', 'failed')),
  details text,
  checked_by_user_id uuid not null references core.users (id),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  unique (prescription_id, check_key)
);
select core.prepare_table('pharmacy.prescription_checks');
create index prescription_checks_prescription_index on pharmacy.prescription_checks (prescription_id);

-- The call to the prescriber when something on a prescription needs confirming.
create table pharmacy.prescriber_contacts (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  prescription_id uuid not null references pharmacy.prescriptions (id) on delete cascade,
  contacted_at timestamptz not null default now(),
  contact_method text not null default 'phone' check (contact_method in ('phone', 'sms', 'whatsapp', 'in_person')),
  outcome text not null check (outcome in ('prescriber_agreed', 'prescription_changed', 'no_answer')),
  note_text text,
  recorded_by_user_id uuid not null references core.users (id),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1
);
select core.prepare_table('pharmacy.prescriber_contacts');
create index prescriber_contacts_prescription_index on pharmacy.prescriber_contacts (prescription_id);

-- One row per label printed for a prescription item; reprints are counted.
create table pharmacy.dispensing_labels (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  prescription_item_id uuid not null references pharmacy.prescription_items (id) on delete cascade,
  label_text text not null,
  printed_at timestamptz not null default now(),
  printed_by_user_id uuid not null references core.users (id),
  reprint_count integer not null default 0 check (reprint_count >= 0),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1
);
select core.prepare_table('pharmacy.dispensing_labels');
create index dispensing_labels_item_index on pharmacy.dispensing_labels (prescription_item_id);

-- When a chronic medicine runs out, when the last one was filled and when
-- the next one is due.
create table pharmacy.refill_schedules (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  customer_id uuid not null references customers.customers (id) on delete cascade,
  product_id uuid not null references catalog.products (id) on delete cascade,
  days_of_supply integer not null check (days_of_supply > 0),
  last_filled_on date,
  next_due_on date not null,
  reminder_status text not null default 'not_due'
    check (reminder_status in ('not_due', 'scheduled', 'sent', 'no_consent', 'collected')),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  unique (customer_id, product_id)
);
select core.prepare_table('pharmacy.refill_schedules');
create index refill_schedules_due_index on pharmacy.refill_schedules (next_due_on);

-- Fridge and cold cabinet readings, one row per recording.
create table pharmacy.temperature_logs (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  branch_id uuid not null references core.branches (id) on delete cascade,
  storage_unit_name text not null,
  recorded_at timestamptz not null default now(),
  temperature_celsius numeric(5,2) not null check (temperature_celsius between -30 and 100),
  is_in_range boolean not null,
  recorded_by_user_id uuid not null references core.users (id),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1
);
select core.prepare_table('pharmacy.temperature_logs');
create index temperature_logs_unit_time_index on pharmacy.temperature_logs (branch_id, storage_unit_name, recorded_at);

-- A recall notice from a supplier or the regulator.
create table pharmacy.recall_notices (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  recall_reference text not null,
  supplier_name text not null,
  marketplace_recall_reference text,
  product_id uuid not null references catalog.products (id),
  batch_number text not null,
  reason_text text not null,
  recall_class text not null default 'class_ii' check (recall_class in ('class_i', 'class_ii', 'class_iii')),
  issued_at timestamptz not null default now(),
  instructions_text text,
  status text not null default 'open' check (status in ('open', 'quarantined', 'returned', 'closed')),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  unique (organization_id, recall_reference)
);
select core.prepare_table('pharmacy.recall_notices');
create index recall_notices_batch_index on pharmacy.recall_notices (product_id, batch_number);

-- What the pharmacy did about a recall: quarantine, contact, return, credit.
create table pharmacy.recall_actions (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  recall_notice_id uuid not null references pharmacy.recall_notices (id) on delete cascade,
  action_type text not null
    check (action_type in ('quarantined', 'patient_contacted', 'returned_to_supplier', 'credit_received')),
  customer_id uuid references customers.customers (id) on delete set null,
  quantity numeric(18,3) check (quantity is null or quantity >= 0),
  done_at timestamptz not null default now(),
  done_by_user_id uuid not null references core.users (id),
  note_text text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1
);
select core.prepare_table('pharmacy.recall_actions');
create index recall_actions_notice_index on pharmacy.recall_actions (recall_notice_id);

-- The licences an inspection asks for, kept with their expiry dates so the
-- readiness checklist can be computed rather than remembered.
create table pharmacy.licence_documents (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  branch_id uuid not null references core.branches (id) on delete cascade,
  licence_type text not null
    check (licence_type in ('premises', 'pharmacist_registration', 'business', 'controlled_permit', 'fire_certificate', 'fridge_calibration')),
  holder_name text not null,
  licence_number text,
  issued_on date,
  expires_on date,
  attachment_id uuid references core.attachments (id) on delete set null,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1
);
select core.prepare_table('pharmacy.licence_documents');
create index licence_documents_expiry_index on pharmacy.licence_documents (branch_id, expires_on);

-- The quality checks done on a delivery, line by line: expiry, packaging,
-- cold chain and quantity.
create table pharmacy.receipt_quality_checks (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  goods_receipt_line_id uuid not null references purchasing.goods_receipt_lines (id) on delete cascade,
  check_key text not null check (check_key in ('expiry', 'packaging', 'cold_chain', 'quantity')),
  result text not null check (result in ('passed', 'warning', 'failed')),
  minimum_temperature_celsius numeric(5,2),
  maximum_temperature_celsius numeric(5,2),
  note_text text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  unique (goods_receipt_line_id, check_key),
  check (minimum_temperature_celsius is null or maximum_temperature_celsius is null or minimum_temperature_celsius <= maximum_temperature_celsius)
);
select core.prepare_table('pharmacy.receipt_quality_checks');

-- A month's claims to one insurer, submitted together and matched to payment.
create table pharmacy.insurance_claim_batches (
  id uuid primary key default core.generate_uuid_v7(),
  organization_id uuid not null references core.organizations (id) on delete cascade,
  insurance_provider_id uuid not null references pharmacy.insurance_providers (id) on delete cascade,
  period_start_on date not null,
  period_end_on date not null,
  status text not null default 'open' check (status in ('open', 'submitted', 'queried', 'paid')),
  submitted_at timestamptz,
  claimed_amount numeric(18,2) not null default 0 check (claimed_amount >= 0),
  paid_amount numeric(18,2) not null default 0 check (paid_amount >= 0),
  deducted_amount numeric(18,2) not null default 0 check (deducted_amount >= 0),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  deleted_at timestamptz,
  row_version bigint not null default 1,
  check (period_start_on <= period_end_on),
  check (paid_amount <= claimed_amount)
);
select core.prepare_table('pharmacy.insurance_claim_batches');
create index insurance_claim_batches_provider_period_index on pharmacy.insurance_claim_batches (insurance_provider_id, period_start_on);

-- Which monthly batch a claim belongs to. Added here because batches arrive
-- with migration 0002; claims submitted before batching keep a null value.
alter table pharmacy.insurance_claims
  add column claim_batch_id uuid references pharmacy.insurance_claim_batches (id) on delete set null;
create index insurance_claims_batch_index on pharmacy.insurance_claims (claim_batch_id);
