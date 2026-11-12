# Contributing to MissionDesk

Thank you for considering a contribution. This page explains how to set up a
development environment, the conventions the code follows and the
contributor licence agreement every contribution requires.

## Before you start

- For anything larger than a small fix, open an issue first so the approach
  can be agreed before you invest time.
- Report security vulnerabilities privately; see [SECURITY.md](SECURITY.md).
- Everyone taking part is expected to follow the
  [Code of Conduct](CODE_OF_CONDUCT.md).

## Contributor licence agreement

Every contribution requires that you sign the contributor licence agreement
in [CLA.md](CLA.md). You keep the copyright in your contribution; the
agreement grants the project the rights it needs to distribute your work
under the AGPL and under other licences, including commercial ones.

Signing happens on the pull request itself: the **CLA Assistant** check asks
you to accept the agreement the first time you open a pull request, and the
check must pass before the pull request can be accepted.

## How accepted changes reach this repository

This repository is a published export of an upstream repository maintained
by the copyright holder. An accepted pull request may therefore be ported
into the upstream repository instead of being merged here directly. When
that happens, your change appears in this repository as part of a later sync
commit, with attribution to you in the commit message.

## Development setup

The application is plain HTML, CSS and JavaScript in `app/`, with no build
step. For most changes a static web server is enough:

```bash
npx serve app
```

To run or build the desktop application you need Node.js 20, a stable Rust
toolchain and the Tauri v1 prerequisites for your platform:

```bash
npm install
npx tauri dev
npx tauri build
```

## Conventions

- **British English** in all code comments, documentation and user-interface
  text: colour, organisation, licence (noun), analyse, behaviour.
- **Licence header.** Every first-party source file starts with the two-line
  SPDX header used throughout the repository.
- **Plain JavaScript.** No frameworks, bundlers or transpilers. Third-party
  libraries are vendored under `app/vendor/` as unmodified upstream builds,
  each with its licence file; nothing is loaded from a CDN at run time.
- **Content Security Policy.** The policy is `script-src 'self'`. Do not add
  inline scripts or inline event handlers (`onclick="..."`); use
  `addEventListener` or the existing `data-action` delegation.
- **Escaping.** Anything that comes from user input, from an opened file or
  from a remote service is escaped (`window.escHtml`) before it is inserted
  into the page.
- **Tauri v1.** The desktop shell uses Tauri v1. Do not introduce Tauri v2
  APIs, configuration fields or crates.
- **Regulatory logic.** A change to a calculation or to a table taken from a
  regulation, AMC or GM cites the document and the section it implements, in
  the code and in the pull request, so that it can be checked at the source.
- **Version.** The version lives in `app/js/version.js` only. A pull request
  that changes behaviour bumps it, following semantic versioning.
- **Commits.** Stage files explicitly by path, so that nothing unintended is
  committed. Use conventional prefixes: `feat:`, `fix:`, `refactor:`,
  `docs:`, `test:`, `chore:`.
- **Merging.** Pull requests are merged with merge commits, not squashed or
  rebased.
