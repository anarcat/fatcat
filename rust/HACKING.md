
## Code Structure

Almost all of the rust code is either auto-generated or "glue" between the
swagger API spec on one side and the SQL schema on the other.

- `./migrations/*/up.sql`: SQL schema
- `./src/database_schema.rs`: autogenerated per-table Diesel schemas
- `./src/database_models.rs`: hand- and macro-generated Rust structs matching
  Diesel schemas, and a small number of row-level helpers
- `./src/api_entity_crud.rs`: "struct-relational-mapping"; trait
  implementations of CRUD (create, read, update, delete) actions for each
  entity model, hitting the database (and building on `database_model` structs)
- `./src/api_server.rs`: one function for each API endpoint, with rust-style
  arguments and return types. mostly calls in to `api_entity_crud`.
- `./src/api_wrappers.rs`: hand- and macro-generated wrapper functions, one per
  API endpoint, that map between API request and return types, and
  rust-idiomatic request and return types (plus API models).
- `./fatcat-api-spec`: autogenerated API models and endpoint types/signatures
- `../fatcat-openapi2.yaml`: OpenAPI 2 specification of API models and
  endpoints

When deciding to use this structure, it wasn't expected that either
`api_wrappers.rs` or `database_models.rs` would need to hand-maintained; both
are verbose and implemented in a very mechanical fashion. The return type
mapping in `api_wrappers` might be necessary, but `database_models.rs` in
particular feels unnecessary; other projects have attempted to completely
automate generation of this file, but it doesn't sound reliable. In particular,
both regular "Row" (queriable) and "NewRow" (insertable) structs need to be
defined.

## Test Structure

- `./tests/test_api_server.rs`: Iron (web framework) level raw HTTP JSON
  request/response tests of API endpoints.

## Updating Schemas

Regenerate API schemas after editing the fatcat-openapi2 schema. This will, as
a side-effect, also run `cargo fmt` on the whole project, so don't run it with
your editor open!

    cargo install cargo-swagger  # uses docker
    ./codegen_openapi2.sh

Update Rust database schema (after changing raw SQL schema):

    diesel database reset
    diesel print-schema > src/database_schema.rs

Debug SQL schema errors (if diesel commands fail):

    psql fatcat_test < migrations/2018-05-12-001226_init/up.sql

## Direct API Interaction

Creating entities via API:

    http --json post localhost:9411/v0/container name=asdf issn=1234-5678
