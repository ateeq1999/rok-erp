create schema if not exists spike;

create table spike.sales (
    id uuid primary key,
    reference text not null,
    amount numeric(18,2) not null,
    insurer_share numeric(18,2)
);