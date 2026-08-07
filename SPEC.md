# SPEC.md - Specification for filipnone.eu

Repository: https://github.com/FilipNone/filipnone.eu
Created: 2026-08-08

## Problem

Build a personal website for filipnone.eu. The site should be fast, secure, and
easy to maintain, built with Rust.

## Intended Users

- Visitors to the public site (content readers).
- The owner as the sole maintainer.

## Required Behavior and User Experience

- Serve a public homepage with personal content.
- Fast page loads (Rust backend, server-side rendering).
- Responsive layout that works on mobile and desktop.
- A small API surface if dynamic data is needed later.

## Architecture and Major Components

- Rust web backend (axum or actix-web, pinned in Cargo.toml).
- Async runtime: tokio.
- Server-side templating (askama or tera).
- Static asset serving.
- Optional: SQL database via sqlx or diesel in a later phase.

## Security and Privacy Requirements

- No secrets committed to the repository.
- Use HTTPS in production.
- Keep dependencies updated (Dependabot).
- CodeQL and dependency review enabled in CI.

## Supported Tool and Dependency Versions

- Rust stable, MSRV 1.88+.
- axum v0.8.9 or actix-web v4.14.0 (choose one, pin it).
- Verify all versions on crates.io before adding. Do not trust LLM memory.

## Non-Goals

- No CMS.
- No user accounts or authentication in the first release.
- No heavy JavaScript framework unless a later phase requires it.

## Acceptance Criteria

- `cargo test` passes.
- `cargo fmt --check` passes.
- `cargo clippy -- -D warnings` passes.
- `cargo build --release` succeeds.
- The server starts and the homepage returns HTTP 200.
- The homepage renders correctly on mobile and desktop.
