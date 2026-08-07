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

## Local documentation (check first)

Documentation for the Rust toolchain and the crates used in this project is
downloaded locally in the `docs/` folder. Before consulting the web, search
these local files first. They are plain-text markdown and are faster and more
reliable than fetching web pages.

- `docs/rust-book/` - The Rust Book (language fundamentals, ownership, error handling).
- `docs/cargo-book/` - The Cargo Book (dependencies, features, profiles, workspaces, commands).
- `docs/rust-reference/` - The Rust Reference (precise language semantics).
- `docs/axum/` - axum web framework READMEs.
- `docs/tokio/` - tokio async runtime README and docs.
- `docs/serde/` - serde serialization README.
- `docs/askama/` - askama templating book.
- `docs/tower/` - tower middleware guides.
- `docs/tower-http/` - tower-http middleware README.
- `docs/tracing/` - tracing logging README.
- `docs/reqwest/` - reqwest HTTP client README.
- `docs/sqlx/` - sqlx database README.

Use `grep` or `file_search` to find the relevant topic in these files before
going online. Only fetch the web when the local docs do not cover the topic.

## Git configuration

- Remote uses SSH: `git@github.com:FilipNone/filipnone.eu.git`.
- Commit author email MUST be the GitHub noreply address:
  `165696124+FilipNone@users.noreply.github.com`.
- Do NOT use the private email.
