---
name: update-deps
description: Update cargo dependencies to their latest major versions one at a time, verifying each with check/build/test and committing individually. Invoke with /update-deps.
disable-model-invocation: false
user-invocable: true
---
# Update Cargo Dependencies

Update each dependency in the root `Cargo.toml` to its latest major version, one at a time, with
verification and individual commits. Then run `cargo update` for minor/patch updates.

## Preconditions

Verify ALL of the following before starting. If any fails, stop and tell the user.

1. **Not on main:** `git branch --show-current` must not be `main`.
2. **Up to date with main:** `git fetch origin main`, then `git rev-parse HEAD` must equal
   `git rev-parse origin/main`. If not: AskUserQuestion Is this branch OK?
3. **Clean working tree:** `git status --porcelain` must produce no output. If not: AskUserQuestion
   commit before proceeding?

## Step 1 — Cargo Update Before Major Bumps

Run `cargo update`, then `cargo check`, `cargo build`, and `cargo test`. Fix issues. If there are
changes, commit with the message `run cargo update`

## Step 2 — Identify Major Version Bumps

Read the root `Cargo.toml`. For each crate under `[dependencies]` (skip `[dev-dependencies]`), run
`cargo search <crate-name> --limit 1` to find the latest published version.

Major update rules:

- `0.x` crates: minor version is the major. `0.3` -> `0.4` is major; `0.3` -> `0.3.5` is not.
- `>=1` crates: only the major matters. `1` -> `2` is major; `1` -> `1.5` is not.

Print the list: `<name>: <current> -> <latest-major>`. If none, skip to Step 3.

## Step 3 — Update Each Dependency

Process each major update sequentially. For each:

1. **Edit Cargo.toml** — change the version to the new major using least-specific semver. Preserve
   features and other attributes.
2. **`cargo update`** — refresh Cargo.lock.
3. **Verify** — run `cargo check`, `cargo build`, `cargo test` in order. If any step fails, read the
   errors and fix them. Repeat until all three pass. Don't give up after one attempt — breakage from
   major bumps is expected. Fix warnings in source code, not in test expectations.
4. **Commit** — stage all changes. Message format: `update <dep> from <old> to <new>` using
   least-specific versions (e.g. `update toml from 0.8 to 0.9`).

## Step 4 — Minor/Patch Updates

1. Run `cargo update`
2. If `Cargo.lock` changed (`git diff --quiet Cargo.lock`), commit it: `run cargo update`
3. If unchanged, tell the user.

## Completion

Summarize: which deps got major updates (from/to), whether cargo update changed anything, total
commits created.
