-- The role the app connects as: it owns nothing and has no BYPASSRLS, so the
-- tenant policies apply to it.
--
-- The role is cluster-wide, so the test binaries that need it race for it, and
-- two races have to be covered. One arrives first and `create role` succeeds
-- (`duplicate_object`); two arrive together and both pass the lookup, so the
-- second one loses on the `pg_authid_rolname_index` unique constraint
-- (`unique_violation`). Catching only the first is what makes this flaky.
--
-- `alter role` is left out for the same reason: the tests read the privilege
-- flags rather than rewriting a row another test may be reading.

do $$
begin
    create role rok_pos_app login password 'rok_pos_app';
exception
    when duplicate_object then null;
    when unique_violation then null;
end
$$;
