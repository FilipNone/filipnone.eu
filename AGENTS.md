# AGENTS.md - Project Guidelines for filipnone.eu

This file tells the coding agent (and any LLM) how to work on this project.
It follows the workflow described in https://christitus.com/my-ai-workflow/:
define the project, build guardrails, work in small phases, and stop at manual gates.

Repository: https://github.com/FilipNone/filipnone.eu
Created: 2026-08-08

## 0. Git Configuration

- Remote uses SSH: `git@github.com:FilipNone/filipnone.eu.git`.
- Commit author email MUST be the GitHub noreply address:
  `165696124+FilipNone@users.noreply.github.com`.
- Do NOT use the private email.
- Set it with: `git config user.email "165696124+FilipNone@users.noreply.github.com"`.

## 1. Global Working Rules (apply everywhere)

- Keep changes focused and small. One reviewable change per pull request.
- Skip filler. Do not add unrelated refactors or "drive-by" edits.
- Preserve unrelated work. Never touch files outside the task scope.
- Run the required checks before declaring a change done (see section 5).
- Stop before destructive actions. Ask before deleting, force-pushing, or rewriting history.
- Do not blindly accept every AI suggestion. Review each one and keep only what is correct.
- The human makes the final decision. AI is an assistant, not the project owner.

## 2. Writing Style Rules (LLM rules)

These rules apply to ALL generated text, code, comments, and documentation:

- Do NOT use the em dash character `—` (U+2014). Use a hyphen, comma, or restructure the sentence.
- Do NOT use emojis or other non-ASCII decorative symbols (checkmarks, crosses, warning signs, bullets, radio/checkbox glyphs, etc.).
- Use plain ASCII equivalents instead: `[x]`, `(off)`, `Yes`/`No`, `*`, `o`, `-`.
- Keep messages concise and impersonal.

## 3. Project Specification (SPEC)

The full specification lives in `SPEC.md`. Summary:

- Problem: a personal website for filipnone.eu.
- Intended users: visitors to the public site; the owner as maintainer.
- Architecture: Rust web backend serving static content and a small API.
- Non-goals: no CMS, no user accounts, no heavy JavaScript framework unless required.
- Acceptance criteria: defined per phase in `ROADMAP.md` and `TASKS.md`.

## 4. Technology Stack (Rust Web)

Rust is production-ready for the web. Verified current versions (2026-08-08):

- Rust toolchain: stable, MSRV 1.88+ (actix-web requires 1.88+).
- Web framework (choose ONE and pin it):
  - `axum` v0.8.9 - modular, tower-based, MSRV 1.80. Recommended default.
  - `actix-web` v4.14.0 - fast, batteries included, MSRV 1.88.
- Async runtime: `tokio`.
- Serialization: `serde` + `serde_json`.
- Templating: `askama` or `tera` (server-side rendering).
- Database (if needed later): `sqlx` (async) or `diesel` (ORM).
- HTTP client: `reqwest`.
- Logging: `tracing` + `tracing-subscriber`.

IMPORTANT: Always verify current versions on crates.io before adding a dependency.
Do not trust the version an LLM remembers from training data. Pin exact versions in `Cargo.toml`.

## 4b. Local Documentation (check first)

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

## 5. Test and Validation Harness (build before features)

Before serious implementation, the project must have:

- Unit tests for core logic (`cargo test`).
- Linting and formatting: `cargo fmt --check` and `cargo clippy -- -D warnings`.
- A clean production build: `cargo build --release`.
- A smoke test: start the server and verify the homepage responds.
- A mobile layout check for any visible web change.

These gates run locally before a pull request and again in CI on the latest commit.

## 6. Implementation Workflow

1. Pick ONE focused task from `TASKS.md`.
2. Inspect the existing code first. Do not guess.
3. Make the smallest change that satisfies the task.
4. Update or add the relevant test.
5. Run the full validation harness (section 5).
6. Open a small pull request (not a direct push to main).
7. Wait for CI and independent review.
8. Resolve all actionable feedback.
9. Manually verify the real result (browser check for web changes).
10. Only then merge and delete the branch.

## 7. Review Process

- Spend more effort on review than on generation.
- For larger changes, run a local review before opening the PR.
- Use an independent reviewer (CodeRabbit, a second agent, or a human) with fresh context.
- The coding session must not grade its own work.
- For every review comment: fix the root cause or explain why the behavior is intentional.
- Repeat until there are no actionable findings left.

## 8. Security and Dependency Checks in CI

Enable GitHub security tools early:

- Dependabot for dependency and GitHub Actions updates.
- CodeQL for code scanning.
- Dependency review for risky dependency changes.
- Required CI checks for tests and production builds.

CI must run on every pull request update. A green check from an older commit does not prove the latest fix is safe.

## 9. Human Merge Gate

Before merging, verify:

- The diff contains only the intended change.
- Tests and builds pass on the latest commit.
- Independent review is complete.
- Actionable review threads are resolved.
- Documentation matches the behavior.
- The feature works in the real target environment.
- Visible changes have been checked on screen.

## 10. Slow Is Smooth, and Smooth Is Fast

- Write the acceptance criteria first.
- Build the tests before the feature.
- Keep phases and pull requests small.
- Wait for CI and independent review.
- Resolve feedback.
- Manually test the real result.

AI makes each step faster. It does not get to skip any of them.

## 11. File Layout

- `AGENTS.md` - this file: how the agent should work.
- `SPEC.md` - what the project must do.
- `ROADMAP.md` - phases with exit criteria.
- `TASKS.md` - small jobs inside the current phase.
- `copilot-instructions.md` - global LLM writing rules.
- `docs/` - local documentation (Rust Book, Cargo Book, Reference, crate docs).
- `Cargo.toml` - pinned dependencies.
- `src/` - Rust source code.
