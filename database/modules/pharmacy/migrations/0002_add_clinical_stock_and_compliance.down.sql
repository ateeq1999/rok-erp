-- Module: pharmacy
-- Migration 0002 rolled back: drop the clinical, stock and compliance tables,
-- in the reverse order they were created, then the column that pointed at them.

drop table pharmacy.receipt_quality_checks;
drop table pharmacy.licence_documents;
drop table pharmacy.recall_actions;
drop table pharmacy.recall_notices;
drop table pharmacy.temperature_logs;
drop table pharmacy.refill_schedules;
drop table pharmacy.dispensing_labels;
drop table pharmacy.prescriber_contacts;
drop table pharmacy.prescription_checks;
drop table pharmacy.interaction_rules;
drop table pharmacy.clinical_notes;
drop table pharmacy.patient_clinical_profiles;
drop table pharmacy.medicine_substitutes;
drop table pharmacy.medicine_details;

alter table pharmacy.insurance_claims drop column claim_batch_id;
drop table pharmacy.insurance_claim_batches;