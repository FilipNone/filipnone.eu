# Coding Instructions

## Writing style

- Do **not** use the em dash character `—` (U+2014) in any generated text, code, or documentation. Use a hyphen `-`, comma, or restructure the sentence instead.
- Do **not** use emojis or other non-ASCII decorative symbols (checkmarks, crosses, warning signs, bullets, radio/checkbox glyphs, etc.) in generated text or documentation. Use plain ASCII equivalents instead (e.g. `[x]`, `(off)`, `Yes`/`No`, `*`, `o`, `-`).

## Project context

This workspace is the `filipnone.eu` personal website project, built in Rust.
Repository: https://github.com/FilipNone/filipnone.eu

Before working on this project, read and follow these files:

- `AGENTS.md` - how the coding agent should work (workflow, review, CI, merge gate).
- `SPEC.md` - what the project must do (requirements and acceptance criteria).
- `ROADMAP.md` - the phases with exit criteria.
- `TASKS.md` - the small jobs inside the current phase.

## Working rules

- Keep changes focused and small. One reviewable change per pull request.
- Skip filler and preserve unrelated work.
- Run the required checks before declaring a change done: `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo build --release`.
- Stop before destructive actions. Ask before deleting, force-pushing, or rewriting history.
- Verify dependency versions on crates.io before adding them. Do not trust LLM memory.
