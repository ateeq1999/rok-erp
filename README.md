# rok-erp

A desktop ERP and point of sale for Tanzanian shops, built on
[GPUI](https://www.gpui.rs/) and PostgreSQL. The first vertical shipped is
**Afya Pharmacy**: prescriptions, dispensing, batches, expiry and recall
control, insurance claims and the till.

The plan lives in [`plan.md`](plan.md); [`roadmap.md`](roadmap.md) records what
is done. `AGENTS.md` is the guide for anyone (or any agent) working here.

## Layout

| Path                       | What it is                                                      |
| -------------------------- | --------------------------------------------------------------- |
| `apps/pharmacy`            | The Afya Pharmacy desktop app.                                   |
| `crates/rok-pos-domain`    | Money, dates and the value types every screen shares.            |
| `crates/rok-pos-database`  | PostgreSQL access: business sessions, models, module installer.  |
| `crates/rok-pos-pharmacy`  | Pharmacy screens, routes and boards.                             |
| `crates/rok-pos-shell`     | Frame, navigation, theme and the top bar.                        |
| `crates/rok-pos-hardware`  | Scales, scanners and other till hardware.                        |
| `database/modules/`        | 25 installable modules: manifests, migrations, permissions.      |
| `database/seeds/`          | The Afya story seed every board is drawn from.                   |
| `tools/migration-verifier` | Applies every migration, reverts it, and applies it again.       |

## How the database is built

Nothing is migrated by hand. Each folder under `database/modules/` carries a
`module.toml` naming the module, its schema, its `depends_on` and its
`[[permissions]]`, plus numbered `NNNN_*.up.sql` / `.down.sql` pairs.

The installer reads every manifest, orders the modules so each one comes after
the modules it needs, runs each pending migration in its own transaction and
records a SHA-256 checksum against it. A migration that has already been
applied can never change underneath the databases that ran it.

```sh
cargo run -p pharmacy -- --install          # install every module, load the story seed
cargo run -p pharmacy -- --install --no-seed  # schema only, for a real pharmacy
cargo run -p migration-verifier -- --modules database/modules
```

`--install` is idempotent: run twice and the second run applies nothing.

## Setup

Rust stable, Docker (or any PostgreSQL 17), and an environment file:

```sh
cp .env.example .env     # then set DATABASE_URL
```

`.env` is gitignored and never committed.

Run the app:

```sh
cargo run -p pharmacy
```

## Checks

CI runs these on every push, with `RUSTFLAGS=-D warnings`:

```sh
cargo fmt --all --check
RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc --workspace --no-deps --all-features
cargo test --workspace --all-features
cargo run -p migration-verifier -- --modules database/modules
```

The database tests each take a database of their own and drop it afterwards,
so they need `DATABASE_URL` pointing at a PostgreSQL server you are willing to
create and drop databases on.

## House rules

- Every public item has a doc comment; no `mod.rs` (a module `foo` with
  children is `foo.rs` beside a `foo/` directory).
- Clippy pedantic is on and CI denies warnings: a warning is fixed, not
  silenced.
- `roadmap.md` is updated when an item in `plan.md` is started or finished.
