# Module map (fixed names other modules may reference)

Every module owns the PostgreSQL schema with the same name as its key.
Foreign keys only ever point at `id` columns of the tables listed here.

| Module key | Depends on | Tables (schema = module key) |
|---|---|---|
| core | — | organizations, branches, devices, users, user_branch_access, permissions, roles, role_permissions, user_roles, installed_modules, applied_migrations, module_dependencies, organization_modules, module_jobs, settings, custom_field_definitions, screen_layouts, approval_rules, number_sequences, studio_publications, translations, audit_events, attachments, notifications, device_sync_cursors, sync_conflicts, change_log |
| catalog | core | units_of_measure, tax_rates, categories, products, product_variants, product_barcodes, price_lists, price_list_items |
| inventory | core, catalog | stock_locations, stock_levels, stock_movements, stock_batches, stock_serial_numbers, stock_transfers, stock_transfer_lines, stock_counts, stock_count_lines, reorder_rules |
| customers | core, catalog | customer_groups, customers, loyalty_programs, loyalty_point_entries, credit_accounts, credit_entries |
| point_of_sale | core, catalog, inventory, customers | shifts, sales, sale_lines, sale_payments, sale_approvals, cash_movements, fiscal_receipts |
| purchasing | core, catalog, inventory | suppliers, supplier_products, purchase_orders, purchase_order_lines, goods_receipts, goods_receipt_lines, supplier_bills |
| promotions | core, catalog, customers, point_of_sale | promotions, promotion_redemptions |
| ai_assistant | core | assistant_settings, page_rules, conversations, messages, proposed_actions, nudge_impressions |
| marketplace | core, catalog, inventory, purchasing | vendors, vendor_documents, vendor_delivery_areas, listings, listing_price_tiers, orders, order_lines, shipments, shipment_events, vendor_ratings, trade_credit_accounts, trade_credit_bills, trade_credit_repayments, payouts, payout_lines |
| delivery | core, customers | riders, delivery_jobs, delivery_job_events, rider_cash_handovers |
| hr | core | employees, employment_contracts, employee_documents, attendance_records, shift_templates, scheduled_shifts, leave_types, leave_balances, leave_requests, payroll_runs, payslips, payslip_lines, salary_advances, commission_rules, commission_earnings, performance_reviews |
| accounting | core | ledger_accounts, journal_entries, journal_lines, money_accounts, expense_categories, expenses, statement_imports, statement_lines, reconciliation_matches |
| quotes_invoices | core, catalog, customers | quotations, quotation_lines, invoices, invoice_lines, invoice_payments |
| online_shop | core, catalog, customers, inventory | storefronts, messaging_channels, inbound_messages, online_orders, online_order_lines |
| crm | core, customers | customer_segments, marketing_consents, campaigns, campaign_recipients |
| restaurant | core, catalog, customers, point_of_sale | dining_areas, dining_tables, modifier_groups, modifiers, product_modifier_groups, kitchen_stations, kitchen_tickets, kitchen_ticket_items, reservations, bill_splits |
| pharmacy | core, catalog, inventory, customers, point_of_sale, purchasing | prescribers, prescriptions, prescription_items, controlled_substance_entries, insurance_providers, insurance_claims |
| fashion | core, catalog, customers, point_of_sale | layaways, layaway_payments, exchange_records |
| hardware | core, catalog, customers, point_of_sale, quotes_invoices | cut_to_length_rules, offcuts, contractor_projects |
| wholesale | core, catalog, inventory, customers, point_of_sale | sales_routes, route_stops, van_loads, van_load_lines, route_visits |
| salon | core, catalog, customers, point_of_sale | bookable_services, staff_service_skills, appointments, appointment_services, service_packages, customer_package_balances |
| bar | core, catalog, inventory, point_of_sale | bar_tabs, pour_measures, bottle_pour_logs |
| bakery | core, catalog, inventory | recipes, recipe_ingredients, production_plans, production_plan_lines, production_batches, waste_records |
| butchery | core, catalog, inventory, point_of_sale | carcasses, carcass_yield_entries, cut_definitions |
| electronics | core, catalog, inventory, customers, point_of_sale | warranties, repair_jobs, repair_job_parts, repair_job_events |
