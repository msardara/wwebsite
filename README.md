# WWebsite

A multilingual wedding website with guest RSVP management and
an admin panel. Built entirely in Rust, compiled to WebAssembly,
and hosted as a static site on GitHub Pages with Supabase as the
managed backend.

Supports English, French, and Italian. Handles multiple event
locations, per-guest dietary preferences, age categories, and
group invitations.

## Tech Stack

| Layer       | Technology                   |
|-------------|------------------------------|
| Frontend    | Leptos 0.6 (Rust/WASM, CSR) |
| Styling     | TailwindCSS (standalone CLI) |
| Backend     | Supabase (PostgreSQL + Auth) |
| Hosting     | GitHub Pages                 |
| Bundler     | Trunk                        |
| Task Runner | Just                         |
| CI/CD       | GitHub Actions               |

## Architecture

### Frontend

The frontend is a single-page application written in Rust using
the [Leptos](https://leptos.dev/) reactive framework, compiled
to WebAssembly and running entirely in the browser (client-side
rendering).

The source is organized as follows:

```
src/
├── components/       # Reusable UI components
│   ├── admin/        #   Admin panel, dashboard, RSVP views
│   ├── common.rs     #   Shared widgets
│   └── layout.rs     #   Page shell and navigation
├── contexts/         # Leptos reactive contexts (DI)
│   ├── admin_context.rs
│   ├── guest_context.rs
│   └── supabase_context.rs
├── i18n/             # Compile-time translations
│   ├── en.rs
│   ├── fr.rs
│   └── it.rs
├── pages/            # Top-level route views
│   ├── admin.rs
│   ├── events.rs
│   ├── home.rs
│   ├── invitation.rs
│   └── rsvp.rs
├── supabase/         # Backend clients and helpers
│   ├── admin_client.rs
│   ├── rpc_client.rs
│   ├── helpers.rs
│   └── error.rs
├── types/            # Domain models (serde-serializable)
│   ├── admin.rs
│   ├── age_category.rs
│   ├── auth.rs
│   ├── dietary.rs
│   ├── guest.rs
│   ├── language.rs
│   └── location.rs
├── constants.rs
├── styles.rs
└── lib.rs            # App root, router, WASM entry point
```

Routing is handled by `leptos_router`. Guest-facing pages
(`/`, `/events`, `/rsvp`) are wrapped in a shared layout that
requires a valid invitation code. The `/invitation` page handles
initial authentication, and `/admin` has its own independent
auth flow.

Internationalization is implemented at compile time: each
language module exposes the same set of string constants, and
the active language is stored in a Leptos signal backed by
`localStorage`.

### Backend

There is no custom backend server. The application talks
directly to [Supabase](https://supabase.com/), which provides:

- **PostgreSQL** as the database, with a strict schema enforced
  by constraints, validation functions, and triggers.
- **PostgREST** as the auto-generated REST API over the
  database.
- **GoTrue** for admin authentication (email/password JWT).

The database schema (`supabase/migrations/`) defines two tables
(`guest_groups` and `guests`) along with domain validation
functions, triggers for timestamp management and invitation code
immutability, and a set of `SECURITY DEFINER` RPC functions for
guest-facing operations.

### Security Model

The application maintains two distinct Supabase clients, each
with different privilege levels:

| Client              | Auth          | Access               |
|---------------------|---------------|----------------------|
| `SupabaseRpcClient` | Anonymous     | RPC only (inv. code) |
| `SupabaseAdminClient`| Authenticated | PostgREST (JWT+RLS) |

**Guest access** is mediated exclusively through PostgreSQL RPC
functions (`authenticate_guest_group`, `get_guests_for_group`,
`save_rsvp`). These functions are declared `SECURITY DEFINER`
and validate the invitation code on every call. Anonymous users
have no direct table access — `REVOKE ALL` is applied on both
tables for the `anon` role.

**Admin access** uses Supabase Auth (email/password). After
sign-in, the JWT token is attached to all requests. Row Level
Security policies restrict direct table operations to the
`authenticated` role.

The `save_rsvp` function handles the entire RSVP submission —
guest creation, updates, deletions, party size adjustment, and
notes — in a single atomic transaction. This avoids partial
writes and race conditions.

### Build Tooling

The project deliberately avoids any Node.js dependency:

- **TailwindCSS** is used via the
  [standalone CLI binary][tw-cli], downloaded and managed
  locally in `.tools/`.
- **Trunk** is the WASM bundler, purpose-built for Rust/WASM
  projects. It compiles the Rust code to WASM, processes the
  `index.html` template, and produces the static `dist/`
  output.
- **Just** is used as the task runner for all build, lint, test,
  and deployment commands.
- **wasm-opt** is applied in release builds for binary size
  optimization (`opt-level = 'z'`, LTO enabled).

All tools are installed automatically by `just setup`.

[tw-cli]: https://tailwindcss.com/blog/standalone-cli

## Rationale for Tech Choices

**Leptos + Rust/WASM** — Provides full type safety across the
entire codebase, from domain models to UI components. Serde
derives ensure that types serialized to the database and
deserialized from API responses are always consistent. Rust's
ownership model eliminates an entire class of runtime errors
common in JavaScript SPAs.

**Client-side rendering** — The site is deployed as a fully
static bundle to GitHub Pages. No server-side rendering is
needed because the content is private (behind invitation codes)
and does not require SEO indexing. This keeps hosting free and
infrastructure minimal.

**Supabase** — Provides a managed PostgreSQL database,
authentication, and an auto-generated REST API without requiring
a custom backend. Business logic lives in PostgreSQL functions
rather than in an application server, reducing the attack
surface and simplifying deployment. The free tier is generous
enough for a wedding website (500 MB database, 50,000 monthly
active users, unlimited API requests), so the entire backend
runs at zero cost.

**RPC-first guest access** — Rather than exposing tables through
PostgREST filters, all guest-facing data access goes through
server-side functions. This makes authorization logic
centralized, auditable, and impossible to bypass from the
client.

**Standalone TailwindCSS** — Removes the Node.js/npm dependency
entirely. The project builds with only Rust tooling and a single
downloaded binary, simplifying CI and local setup.

**Just** — A simpler, more predictable alternative to Make. No
implicit rules, no tab sensitivity, and cross-platform support.

## Prerequisites

- [Just](https://just.systems/) command runner

All other tools — including Rust itself, Trunk, TailwindCSS CLI,
and wasm-opt — are installed automatically via `just setup`.

The required Rust toolchain and compilation targets are declared
in `rust-toolchain.toml`. Rustup reads this file automatically
and installs the correct toolchain on first invocation.

## Supabase Setup

The backend requires a [Supabase](https://supabase.com/)
project. If you do not already have one:

1. Create an account at https://supabase.com.
2. Click **New Project**. Choose a name, set a database
   password, and select a region.
3. Wait for the project to finish provisioning (usually
   under a minute).

Once the project is ready, configure the local environment,
apply the database schema, and create an admin user:

```sh
# Write SUPABASE_URL and SUPABASE_PUBLISHABLE_KEY to .env
just db-configure

# Push the schema from supabase/migrations/ to the database
just db-migrate

# Create an admin user for the /admin panel
just db-create-admin
```

`just db-configure` reads the project reference and API keys
from the Supabase CLI and writes them to `.env`. The Rust
build embeds these values at compile time via `env!()`.

## Getting Started

```sh
# Install Rust, Trunk, TailwindCSS CLI, wasm-opt, and
# build the CSS
just setup

# Configure Supabase credentials (see above)
just db-configure

# Push the database schema
just db-migrate

# Start the development server (opens browser)
just dev
```

## Production Build

```sh
just build-release
```

Output is written to `dist/`.

## CI/CD

Continuous integration runs on every push and pull request
against `main`. The pipeline includes formatting
checks, linting (Clippy), tests, security audits, dependency
license checks, typo detection, and a full WASM build
verification.

Deployments to GitHub Pages are triggered automatically on tag
pushes matching `v*`, or manually through `workflow_dispatch`.

## Project Commands

Run `just` with no arguments to list all available commands.
Key commands:

| Command               | Description                       |
|-----------------------|-----------------------------------|
| `just setup`          | Install all tools and dependencies|
| `just dev`            | Start the development server      |
| `just build-release`  | Production build with optimization|
| `just lint`           | Run all linters (fmt, clippy, etc)|
| `just test`           | Run the test suite                |
| `just ci`             | Simulate CI pipeline locally      |
| `just db-configure`   | Set Supabase credentials          |
| `just db-migrate`     | Apply database migrations         |
| `just db-create-admin`| Create admin user                 |
| `just clean`          | Clean all build artifacts         |

## License

See [LICENSE](LICENSE).
