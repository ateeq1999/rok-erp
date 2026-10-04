-- Phase 0 spike 2: a tenant table and the role the app connects as.
--
-- The policies read `app.organization_id`, which `in_business` sets for the
-- transaction. `rok_pos_app` owns nothing and has no BYPASSRLS, so the policies
-- apply to it; the role is cluster-wide, so it is created only once.

create schema if not exists spike;

create table spike.batches (
    id uuid primary key,
    organization_id uuid not null,
    batch_number text not null,
    expiry_on date not null,
    amount numeric(18,2) not null default 0
);

alter table spike.batches enable row level security;
alter table spike.batches force row level security;

create policy batches_read_own_business on spike.batches
    for select
    using (organization_id = nullif(current_setting('app.organization_id', true), '')::uuid);

create policy batches_write_own_business on spike.batches
    for insert
    with check (organization_id = nullif(current_setting('app.organization_id', true), '')::uuid);

create policy batches_change_own_business on spike.batches
    for update
    using (organization_id = nullif(current_setting('app.organization_id', true), '')::uuid)
    with check (organization_id = nullif(current_setting('app.organization_id', true), '')::uuid);

-- The role is cluster-wide, so tests that run at the same time race for it.
-- `alter role` is left out for the same reason: the test checks the flag rather
-- than rewriting a shared row.
do $$
begin
    create role rok_pos_app login password 'rok_pos_app';
exception
    when duplicate_object then null;
end
$$;

grant usage on schema spike to rok_pos_app;
grant select, insert, update on spike.batches to rok_pos_app;