# Agent Rules — orchestrator-test

This is a minimal Rust test repository used to validate the orchestrator end-to-end workflow.

## Your Job

You will receive a task describing a Rust function to implement. Complete it fully and commit.

## Rules

1. **Read the task carefully.** Implement exactly what is asked — no more, no less.
2. **Edit `src/lib.rs`** to add the requested public function(s).
3. **Edit `src/tests.rs`** to add the requested unit tests inside the existing `#[cfg(test)] mod tests` block.
4. **Run quality gates** in order:
   - `cargo fmt` — format code (required)
   - `cargo clippy -- -D warnings` — fix all warnings
   - `cargo test` — all tests must pass
5. **Commit everything** with a conventional commit message:
   ```
   git add -A
   git commit -m "feat: <short description>"
   ```
6. **Do NOT push.** The orchestrator handles pushing and PR creation.
7. If you make no code changes, create a placeholder commit:
   ```
   git commit --allow-empty -m "chore: no changes required"
   ```

## Project Structure

- `src/lib.rs` — public library functions
- `src/tests.rs` — unit tests (included via `#[cfg(test)] mod tests` in lib.rs)
- `Cargo.toml` — standard Rust package, edition 2021
