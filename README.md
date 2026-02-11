# WWebsite

## Features

- 🌍 **Multilingual Support**: English, French, and Italian with automatic language detection
- 📍 **Multi-Location**: Separate event information for Sardinia and Tunisia
- 📝 **RSVP System**: Guest lookup and response management with dietary restrictions
- 📷 **Photo Gallery**: Showcase memories with an elegant gallery
- 👑 **Admin Panel**: Manage guests, RSVPs, content, and photos
- 🔒 **Secure**: Supabase backend with Row Level Security
- 📱 **Responsive**: Mobile-first design that works on all devices
- ⚡ **Fast**: Static WASM deployment on GitHub Pages

## Tech Stack

- **Frontend**: Leptos (Rust WASM framework)
- **Styling**: TailwindCSS
- **Backend**: Supabase (PostgreSQL + Auth + Storage)
- **Deployment**: GitHub Pages
- **Build Tool**: Trunk

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://rustup.rs/) (latest stable)
- [Trunk](https://trunkrs.dev/) - WASM build tool
- [Node.js](https://nodejs.org/) - For TailwindCSS
- [Just](https://just.systems/) - Command runner (recommended)
- A [Supabase](https://supabase.com/) account (free tier)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install WASM target

```bash
rustup target add wasm32-unknown-unknown
```

### Install Just

Just is a command runner that makes development easier:

**macOS:**
```bash
brew install just
```

**Linux:**
```bash
cargo install just
```

**Windows:**
```bash
cargo install just
```

## Development Tooling

This project includes comprehensive tooling for code quality, security, and CI/CD:

- **just** - Command runner with all common tasks
- **cargo-audit** - Security vulnerability scanning
- **cargo-deny** - License and dependency checking
- **cargo-machete** - Unused dependency detection
- **typos-cli** - Spell checking
- **clippy** - Rust linting
- **rustfmt** - Code formatting

### Quick Setup with Just

```bash
# Install all dependencies and tools and start dev server
just dev
```

### 3. Set Up Supabase

#### Create a Supabase Project

1. Go to [supabase.com](https://supabase.com)
2. Create a new project
3. Wait for the database to be provisioned

#### Run Database Migrations

```bash
just db-configure
just db-migrate
```

## Building for Production

### Build the Project

```bash
just build-release
```

This creates optimized WASM files in the `dist/` directory.

**Made with ❤️ and 🦀 Rust**
