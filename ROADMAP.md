# ROADMAP.md - Roadmap for filipnone.eu

Repository: https://github.com/FilipNone/filipnone.eu
Created: 2026-08-08

Each phase delivers something concrete, states its dependencies, and defines
what must be true before moving on.

## Phase 0 - Project Scaffolding

Delivers: a working Rust project skeleton with the test harness in place.

- Initialize Cargo project.
- Add pinned dependencies (axum or actix-web, tokio, serde, templating).
- Add `cargo fmt`, `cargo clippy`, and `cargo test` config.
- Add CI workflow (tests, lint, production build).
- Enable Dependabot, CodeQL, and dependency review.

Exit criteria:
- `cargo test`, `cargo fmt --check`, `cargo clippy -- -D warnings` all pass.
- `cargo build --release` succeeds.
- CI is green on the latest commit.

## Phase 1 - Homepage

Delivers: a public homepage served by the Rust backend.

- Add a root route returning the homepage.
- Add server-side templating for the page.
- Add static asset serving (CSS).
- Add a smoke test that the homepage returns HTTP 200.

Exit criteria:
- Homepage loads in a browser.
- Mobile layout check passes.
- Smoke test passes.

## Phase 2 - Content and Styling

Delivers: styled, responsive content pages.

- Add content sections to the homepage.
- Add responsive CSS.
- Add a mobile layout check to the harness.

Exit criteria:
- Content renders correctly on mobile and desktop.
- All validation gates pass.

## Phase 3 - Optional API / Data

Delivers: a small API surface if dynamic data is required.

- Add JSON endpoints.
- Add database integration if needed (sqlx or diesel).

Exit criteria:
- API endpoints tested.
- All validation gates pass.

## Future Phases

- Deployment to production.
- Analytics and monitoring.
- Additional content types.
