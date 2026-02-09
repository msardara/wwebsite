# Wedding Website - Justfile
# Build automation for Leptos/WASM project

# Load environment variables from .env file
set dotenv-load

# Default recipe - show available commands
default:
    @just --list

# ============================================================================
# VARIABLES
# ============================================================================

# Build profile (debug or release)
profile := env("PROFILE", "debug")

# Target architecture
target := env("TARGET", "wasm32-unknown-unknown")

# Tools directory for local binary installations
tools_dir := ".tools"

# ============================================================================
# SETUP & INSTALLATION
# ============================================================================

# Install all required tools and dependencies
setup: install-wasm-target install-trunk install-wasm-opt _install-npm-deps build-css
    @echo "✅ Setup complete! Run 'just dev' to start developing"

# Internal recipe for npm install (only runs if needed)
_install-npm-deps:
    @if [ ! -d "node_modules" ]; then \
        echo "📦 Installing Node.js dependencies..."; \
        npm install; \
    else \
        echo "✅ Node.js dependencies already installed"; \
    fi

# Install WASM target (idempotent - skips if already installed)
install-wasm-target:
    @if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then \
        echo "✅ WASM target already installed"; \
    else \
        echo "📦 Installing WASM target..."; \
        rustup target add wasm32-unknown-unknown; \
        echo "✅ WASM target installed successfully!"; \
    fi

# Install Trunk build tool (idempotent - skips if already installed)
install-trunk:
    #!/usr/bin/env sh
    if [ -f {{tools_dir}}/bin/trunk ]; then
        echo "✅ Trunk already installed: $({{tools_dir}}/bin/trunk --version)"
        exit 0
    fi
    echo "📦 Installing Trunk from GitHub releases to {{tools_dir}}/..."
    mkdir -p {{tools_dir}}/bin
    PROJECT_DIR=$(pwd)
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then
        ARCH="x86_64"
    elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ARCH="aarch64"
    else
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
    fi
    if [ "$OS" = "darwin" ]; then
        OS="apple-darwin"
    elif [ "$OS" = "linux" ]; then
        OS="unknown-linux-gnu"
    else
        echo "❌ Unsupported OS: $OS"
        exit 1
    fi
    VERSION="v0.21.14"
    FILENAME="trunk-$ARCH-$OS.tar.gz"
    URL="https://github.com/trunk-rs/trunk/releases/download/$VERSION/$FILENAME"
    echo "Downloading $URL..."
    cd /tmp
    curl -L -o "$FILENAME" "$URL"
    tar xzf "$FILENAME"
    cp trunk "$PROJECT_DIR/{{tools_dir}}/bin/"
    rm -f trunk "$FILENAME"
    cd "$PROJECT_DIR"
    echo "✅ Trunk installed successfully: $({{tools_dir}}/bin/trunk --version)"

# Install wasm-opt from binaryen releases (idempotent - skips if already installed)
install-wasm-opt:
    #!/usr/bin/env sh
    if [ -f {{tools_dir}}/bin/wasm-opt ]; then
        echo "✅ wasm-opt already installed: $({{tools_dir}}/bin/wasm-opt --version)"
        exit 0
    fi
    echo "📦 Installing wasm-opt from binaryen to {{tools_dir}}/..."
    mkdir -p {{tools_dir}}/bin
    PROJECT_DIR=$(pwd)
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then
        ARCH="x86_64"
    elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ARCH="arm64"
    else
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
    fi
    if [ "$OS" = "darwin" ]; then
        OS="macos"
    elif [ "$OS" = "linux" ]; then
        OS="linux"
    else
        echo "❌ Unsupported OS: $OS"
        exit 1
    fi
    VERSION="version_117"
    FILENAME="binaryen-$VERSION-$ARCH-$OS.tar.gz"
    URL="https://github.com/WebAssembly/binaryen/releases/download/$VERSION/$FILENAME"
    echo "Downloading $URL..."
    cd /tmp
    curl -L -o "$FILENAME" "$URL"
    tar xzf "$FILENAME"
    mkdir -p "$PROJECT_DIR/{{tools_dir}}/lib"
    cp "binaryen-$VERSION/bin/wasm-opt" "$PROJECT_DIR/{{tools_dir}}/bin/"
    if [ -d "binaryen-$VERSION/lib" ]; then
        cp -r "binaryen-$VERSION/lib/"* "$PROJECT_DIR/{{tools_dir}}/lib/"
    fi
    rm -rf "binaryen-$VERSION" "$FILENAME"
    cd "$PROJECT_DIR"
    echo "✅ wasm-opt installed successfully: $({{tools_dir}}/bin/wasm-opt --version)"

# Install cargo-llvm-cov for code coverage (idempotent - skips if already installed)
install-cargo-llvm-cov:
    @if cargo install --list | grep -q "^cargo-llvm-cov v"; then \
        echo "✅ cargo-llvm-cov already installed: $$(cargo llvm-cov --version | head -n1)"; \
    else \
        echo "📦 Installing cargo-llvm-cov..."; \
        cargo install cargo-llvm-cov; \
        echo "✅ cargo-llvm-cov installed successfully!"; \
    fi

# Install development tools
install-tools:
    @echo "🔧 Installing development tools..."
    @cargo install --list | grep -q "^cargo-audit v" || cargo install cargo-audit
    @cargo install --list | grep -q "^cargo-machete v" || cargo install cargo-machete
    @cargo install --list | grep -q "^cargo-deny v" || cargo install cargo-deny
    @cargo install --list | grep -q "^cargo-sort v" || cargo install cargo-sort
    @cargo install --list | grep -q "^typos-cli v" || cargo install typos-cli
    @cargo install --list | grep -q "^cargo-llvm-cov v" || cargo install cargo-llvm-cov
    @echo "✅ Tools installed!"

# Update all dependencies
update:
    cargo update
    npm update

# ============================================================================
# DEVELOPMENT
# ============================================================================

# Run development server with hot reload
dev: install-wasm-target install-trunk build-css
    @echo "🚀 Starting development server..."
    @if [ -f .env ]; then \
        set -a && . ./.env && set +a && {{tools_dir}}/bin/trunk serve --open; \
    else \
        echo "⚠️  Warning: .env file not found. Run 'just db-configure' to set up environment variables."; \
        {{tools_dir}}/bin/trunk serve --open; \
    fi

# Run development server without opening browser
serve: install-wasm-target install-trunk build-css
    @if [ -f .env ]; then \
        set -a && . ./.env && set +a && {{tools_dir}}/bin/trunk serve; \
    else \
        echo "⚠️  Warning: .env file not found. Run 'just db-configure' to set up environment variables."; \
        {{tools_dir}}/bin/trunk serve; \
    fi

# Build CSS and watch for changes
watch-css: _install-npm-deps
    npm run watch:css

# Build CSS once
build-css: _install-npm-deps
    @if [ ! -f "style/output.css" ] || [ "style/main.css" -nt "style/output.css" ]; then \
        echo "🎨 Building CSS..."; \
        npm run build:css; \
    else \
        echo "✅ CSS already built and up to date"; \
    fi

# Run both dev server and CSS watch in parallel (requires 'concurrently' npm package)
dev-all: install-wasm-target install-trunk _install-npm-deps
    @npm list concurrently >/dev/null 2>&1 || npm install -D concurrently
    npx concurrently {{tools_dir}}/bin/trunk serve" "npm run watch:css"

# ============================================================================
# BUILD
# ============================================================================

# Build the project (debug mode)
# Build project
build: install-trunk build-css
    @echo "🔨 Building project..."
    @if [ -f .env ]; then \
        set -a && . ./.env && set +a && {{tools_dir}}/bin/trunk build; \
    else \
        echo "⚠️  Warning: .env file not found. Run 'just db-configure' to set up environment variables."; \
        {{tools_dir}}/bin/trunk build; \
    fi

# Build for production (optimized)
build-release: install-trunk _install-npm-deps
    @echo "🔨 Building for production..."
    npm run build:css:prod
    @if [ -f .env ]; then \
        set -a && . ./.env && set +a && {{tools_dir}}/bin/trunk build --release; \
    else \
        echo "⚠️  Warning: .env file not found. Run 'just db-configure' to set up environment variables."; \
        {{tools_dir}}/bin/trunk build --release; \
    fi

    @just _optimize-wasm

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    trunk clean
    rm -rf dist/
    rm -rf style/output.css
    @echo "✅ Cleaned!"

# Clean and rebuild
rebuild: clean build

# ============================================================================
# CODE QUALITY
# ============================================================================

# Check code without building
check:
    @echo "🔍 Checking code..."
    cargo check --all-targets --all-features

# Format code
fmt:
    @echo "🎨 Formatting code..."
    cargo fmt

# Check if code is formatted
fmt-check:
    @echo "🔍 Checking code format..."
    cargo fmt -- --check

# Run clippy linter
clippy:
    @echo "📎 Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

# Check for typos in code and docs
typos:
    @echo "🔤 Checking for typos..."
    @if command -v typos >/dev/null 2>&1; then \
        typos; \
    else \
        echo "⚠️  typos not installed. Run 'just install-tools' to install it."; \
    fi

# Fix typos automatically
typos-fix:
    @echo "🔤 Fixing typos..."
    @if command -v typos >/dev/null 2>&1; then \
        typos --write-changes; \
    else \
        echo "⚠️  typos not installed. Run 'just install-tools' to install it."; \
    fi

# Sort Cargo.toml dependencies
sort:
    @echo "📋 Sorting dependencies..."
    @if cargo install --list | grep -q "^cargo-sort v"; then \
        cargo sort; \
    else \
        echo "⚠️  cargo-sort not installed. Run 'just install-tools' to install it."; \
    fi

# Check if dependencies are sorted
sort-check:
    @echo "🔍 Checking dependency order..."
    @if cargo install --list | grep -q "^cargo-sort v"; then \
        cargo sort --check; \
    else \
        echo "⚠️  cargo-sort not installed. Run 'just install-tools' to install it."; \
    fi

# ============================================================================
# SECURITY & DEPENDENCIES
# ============================================================================

# Check for security vulnerabilities
audit:
    @echo "🔒 Checking for vulnerabilities..."
    @if cargo install --list | grep -q "^cargo-audit v"; then \
        cargo audit; \
    else \
        echo "⚠️  cargo-audit not installed. Run 'just install-tools' to install it."; \
    fi

# Check for unused dependencies
unused:
    @echo "🔍 Checking for unused dependencies..."
    @if cargo install --list | grep -q "^cargo-machete v"; then \
        cargo machete; \
    else \
        echo "⚠️  cargo-machete not installed. Run 'just install-tools' to install it."; \
    fi

# Check licenses of dependencies
licenses:
    @echo "📜 Checking licenses..."
    @if cargo install --list | grep -q "^cargo-deny v"; then \
        cargo deny check licenses; \
    else \
        echo "⚠️  cargo-deny not installed. Run 'just install-tools' to install it."; \
    fi

# Check for security advisories (alias for audit)
vuln: audit

# Run all dependency checks
check-deps: unused licenses audit

# ============================================================================
# TESTING
# ============================================================================

# Run all tests
test:
    @echo "🧪 Running tests..."
    SUPABASE_URL=https://dummy.supabase.co SUPABASE_PUBLISHABLE_KEY=dummy-key cargo test --all-features

# Run tests with coverage
test-coverage: install-cargo-llvm-cov
    @echo "🧪 Running tests with coverage..."
    SUPABASE_URL=https://dummy.supabase.co SUPABASE_PUBLISHABLE_KEY=dummy-key cargo llvm-cov --lcov --output-path lcov.info --all-features

# Build tests without running
test-build:
    @echo "🔨 Building tests..."
    cargo test --no-run --all-features

# ============================================================================
# LINTING - RUN ALL CHECKS
# ============================================================================

# Run all linting checks
lint: fmt-check clippy sort-check typos

# Run all checks (lint + security + tests)
check-all: lint audit unused test

# Fix all auto-fixable issues
fix: fmt sort typos-fix

# ============================================================================
# DOCUMENTATION
# ============================================================================

# Generate documentation
doc:
    @echo "📚 Generating documentation..."
    cargo doc --no-deps --all-features

# Generate and open documentation
doc-open:
    @echo "📚 Generating and opening documentation..."
    cargo doc --no-deps --all-features --open

# ============================================================================
# DEPLOYMENT
# ============================================================================

# Optimize WASM files with wasm-opt
_optimize-wasm: install-wasm-opt
    #!/usr/bin/env sh
    echo "🔧 Optimizing WASM with wasm-opt..."
    if [ -f {{tools_dir}}/bin/wasm-opt ]; then
        WASM_OPT="{{tools_dir}}/bin/wasm-opt"
    elif command -v wasm-opt >/dev/null 2>&1; then
        WASM_OPT="wasm-opt"
    else
        echo "⚠️  wasm-opt not found"
        exit 1
    fi
    for wasm in dist/*.wasm; do
        if [ -f "$wasm" ]; then
            echo "Optimizing $wasm..."
            $WASM_OPT -Oz "$wasm" -o "$wasm.tmp" && mv "$wasm.tmp" "$wasm"
        fi
    done
    echo "✅ WASM optimization complete!"

# Serve the production build locally for testing
serve-dist:
    @echo "🌐 Serving production build..."
    @if [ -d "dist" ]; then \
        cd dist && python3 -m http.server 8080; \
    else \
        echo "❌ No dist directory found. Run 'just build-release' first."; \
    fi

# ============================================================================
# DATABASE (SUPABASE) - Requires Supabase CLI
# ============================================================================

# Check if Supabase CLI is installed
# Install Supabase CLI (idempotent - skips if already installed)
install-supabase-cli:
    @if command -v supabase >/dev/null 2>&1; then \
        echo "✅ Supabase CLI already installed: $$(supabase --version)"; \
    elif command -v brew >/dev/null 2>&1; then \
        echo "📦 Installing Supabase CLI via Homebrew..."; \
        brew install supabase/tap/supabase; \
        echo "✅ Supabase CLI installed successfully!"; \
    elif command -v npm >/dev/null 2>&1; then \
        echo "📦 Installing Supabase CLI via npm..."; \
        npm install -g supabase; \
        echo "✅ Supabase CLI installed successfully!"; \
    else \
        echo "❌ No package manager found (brew or npm)"; \
        echo ""; \
        echo "Please install Supabase CLI manually:"; \
        echo "  macOS:   brew install supabase/tap/supabase"; \
        echo "  npm:     npm install -g supabase"; \
        echo "  Windows: scoop install supabase"; \
        echo ""; \
        echo "See: https://supabase.com/docs/guides/cli/getting-started"; \
        exit 1; \
    fi

# Login to Supabase CLI (run once)
db-login: install-supabase-cli
    @echo "🔐 Logging in to Supabase..."
    @echo "This will open a browser window to authenticate."
    @echo ""
    supabase login

# Link to Supabase project
db-link: install-supabase-cli db-login
    @echo "🔗 Linking to Supabase project..."
    @echo ""
    @supabase link || \
        (echo ""; \
         echo "❌ Failed to link. You may need to login first:"; \
         echo "   just db-login"; \
         exit 1)
    @echo "✅ Successfully linked to project!"

# Run database migrations
db-migrate: install-supabase-cli
    @echo "🔄 Running database migrations..."
    @echo "Pushing migrations to database..."
    @if supabase db push --include-all; then \
        echo "✅ Migrations complete!"; \
        echo ""; \
        echo "📝 Database updated with latest schema changes"; \
    else \
        echo ""; \
        echo "❌ Migration failed."; \
        echo ""; \
        echo "Manual migration:"; \
        echo "  1. Open: https://stmfsurqatlfqieuxclt.supabase.co"; \
        echo "  2. Go to SQL Editor"; \
        echo "  3. Run migrations from supabase/migrations/"; \
        exit 1; \
    fi

# Configure .env file automatically from Supabase CLI
db-configure: install-supabase-cli db-link
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🔧 Configuring environment variables from Supabase..."
    echo ""
    if [ ! -f "supabase/.temp/project-ref" ]; then
        echo "❌ Project not linked. Run: just db-link"
        exit 1
    fi
    PROJECT_REF=$(cat supabase/.temp/project-ref)
    echo "Project: $PROJECT_REF"
    echo "Fetching API keys..."
    PUB_KEY=$(supabase projects api-keys --project-ref $PROJECT_REF 2>/dev/null | grep "sb_publishable" | head -1 | awk '{print $3}')
    if [ -z "$PUB_KEY" ]; then
        echo "❌ Failed to fetch API keys. You may need to login again: just db-login"
        exit 1
    fi
    echo "SUPABASE_URL=https://$PROJECT_REF.supabase.co" > .env
    echo "SUPABASE_PUBLISHABLE_KEY=$PUB_KEY" >> .env
    echo ""
    echo "✅ .env file configured!"
    echo ""
    echo "Contents:"
    echo "  SUPABASE_URL=https://$PROJECT_REF.supabase.co"
    echo "  SUPABASE_PUBLISHABLE_KEY=$PUB_KEY"
    echo ""
    echo "Next: Rebuild the project with 'just build' or 'just dev'"

# Create admin user in Supabase Auth
db-create-admin: install-supabase-cli db-link
    #!/usr/bin/env bash
    set -euo pipefail
    echo "👤 Creating admin user..."
    echo ""
    if [ ! -f "supabase/.temp/project-ref" ]; then
        echo "❌ Project not linked. Run: just db-link"
        exit 1
    fi
    PROJECT_REF=$(cat supabase/.temp/project-ref)
    EMAIL="mauro.sardara@gmail.com"
    PASSWORD=$(openssl rand -base64 16)
    echo "Creating user with email: $EMAIL"
    echo ""
    # Get the anon key
    ANON_KEY=$(supabase projects api-keys --project-ref $PROJECT_REF 2>/dev/null | grep "anon" | head -1 | awk '{print $3}')
    if [ -z "$ANON_KEY" ]; then
        echo "❌ Failed to fetch API key. You may need to login again: just db-login"
        exit 1
    fi
    # Create user via Supabase Auth API
    RESPONSE=$(curl -s -X POST "https://$PROJECT_REF.supabase.co/auth/v1/signup" \
        -H "apikey: $ANON_KEY" \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\"}")
    # Check if user was created or already exists
    if echo "$RESPONSE" | grep -q "email" || echo "$RESPONSE" | grep -q "already"; then
        echo "✅ Admin user created/updated!"
    else
        echo "⚠️  Response: $RESPONSE"
        echo "User may already exist or there was an issue."
    fi
    echo ""
    echo "Login credentials:"
    echo "  Email: $EMAIL"
    echo "  Password: $PASSWORD"
    echo ""
    echo "⚠️  IMPORTANT: Save this password securely!"
    echo ""
    echo "📧 Check your email to confirm the account (if email confirmation is enabled)"
    echo ""

# Show instructions for getting API keys (legacy - use db-configure instead)
db-show-keys:
    @echo "🔑 Getting Your Supabase API Keys"
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo ""
    @echo "✨ TIP: Use 'just db-configure' to automatically set up .env!"
    @echo ""
    @echo "Or manually:"
    @echo "1. Run: supabase projects api-keys --project-ref stmfsurqatlfqieuxclt"
    @echo "2. Copy the publishable key"
    @echo "3. Update your .env file:"
    @echo "   SUPABASE_URL=https://stmfsurqatlfqieuxclt.supabase.co"
    @echo "   SUPABASE_PUBLISHABLE_KEY=<paste-key-here>"
    @echo ""
    @echo "4. Rebuild the project: just build"

# ============================================================================
# PROJECT INFO
# ============================================================================

# Check which tools are installed
check-installed:
    @echo "🔍 Checking installed tools..."
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @printf "Rust:          "; rustc --version 2>/dev/null || echo "❌ NOT INSTALLED"
    @printf "Cargo:         "; cargo --version 2>/dev/null || echo "❌ NOT INSTALLED"
    @printf "WASM target:   "; rustup target list | grep -q "wasm32-unknown-unknown (installed)" && echo "✅ installed" || echo "❌ NOT INSTALLED - run 'just install-wasm-target'"
    @printf "Node:          "; node --version 2>/dev/null || echo "❌ NOT INSTALLED"
    @printf "npm:           "; npm --version 2>/dev/null || echo "❌ NOT INSTALLED"
    @printf "Trunk:         "; if [ -f {{tools_dir}}/bin/trunk ]; then {{tools_dir}}/bin/trunk --version; else echo "❌ NOT INSTALLED - run 'just install-trunk'"; fi
    @printf "Supabase CLI:  "; supabase --version 2>/dev/null || echo "❌ NOT INSTALLED - run 'just install-supabase-cli'"
    @printf "Just:          "; just --version 2>/dev/null || echo "✅ (you're using it)"
    @echo ""
    @echo "Development tools (optional):"
    @printf "cargo-audit:   "; cargo audit --version 2>/dev/null || echo "❌ run 'just install-tools'"
    @printf "cargo-deny:    "; cargo deny --version 2>/dev/null || echo "❌ run 'just install-tools'"
    @printf "cargo-machete: "; cargo machete --version 2>/dev/null || echo "❌ run 'just install-tools'"
    @printf "typos:         "; typos --version 2>/dev/null || echo "❌ run 'just install-tools'"

# Show project information
info:
    @echo "📋 Wedding Website Project Info"
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo "Rust version:    $(rustc --version)"
    @echo "Cargo version:   $(cargo --version)"
    @echo "Node version:    $(node --version)"
    @echo "Trunk version:   $(trunk --version 2>/dev/null || echo 'not installed')"
    @echo "Supabase CLI:    $(supabase --version 2>/dev/null || echo 'not installed')"
    @echo "Build profile:   {{profile}}"
    @echo "Target:          {{target}}"
    @echo ""
    @echo "Project status:  Phase 3 Complete (RSVP System)"
    @echo "Next milestone:  Phase 4 (Admin Dashboard)"

# Show project statistics
stats:
    @echo "📊 Project Statistics"
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo "Rust files:      $(find src -name '*.rs' 2>/dev/null | wc -l)"
    @echo "Lines of code:   $(find src -name '*.rs' 2>/dev/null -exec cat {} \; | wc -l)"
    @echo "Dependencies:    $(grep -c '^[a-zA-Z]' Cargo.toml | tail -1)"
    @echo "Pages:           $(find src/pages -name '*.rs' 2>/dev/null | wc -l)"
    @echo "Components:      $(find src/components -name '*.rs' 2>/dev/null | wc -l)"

# ============================================================================
# UTILITIES
# ============================================================================

# Open project in browser
open:
    @echo "🌐 Opening http://127.0.0.1:3000 in browser..."
    @open http://127.0.0.1:3000 2>/dev/null || xdg-open http://127.0.0.1:3000 2>/dev/null || echo "Please open http://127.0.0.1:3000 manually"

# Watch for file changes and rebuild
watch:
    @if command -v cargo-watch >/dev/null 2>&1; then \
        cargo watch -x check -x test; \
    else \
        echo "❌ cargo-watch not installed. Install with: cargo install cargo-watch"; \
    fi

# Count lines of code
loc:
    @echo "📏 Lines of code by type:"
    @echo "Rust:       $(find src -name '*.rs' 2>/dev/null -exec cat {} \; | wc -l)"
    @echo "CSS:        $(find style -name '*.css' 2>/dev/null -exec cat {} \; | wc -l)"
    @echo "HTML:       $(find . -name '*.html' 2>/dev/null -exec cat {} \; | wc -l)"
    @echo "Markdown:   $(find . -name '*.md' 2>/dev/null -exec cat {} \; | wc -l)"

# Check disk space used by build artifacts
disk-usage:
    @echo "💾 Disk usage:"
    @du -sh target/ 2>/dev/null || echo "No target directory"
    @du -sh dist/ 2>/dev/null || echo "No dist directory"
    @du -sh node_modules/ 2>/dev/null || echo "No node_modules directory"

# ============================================================================
# CI/CD SIMULATION
# ============================================================================

# Simulate CI pipeline locally
ci: check-all build-release
    @echo "✅ CI pipeline complete!"

# Pre-commit checks (fast)
pre-commit: fmt-check clippy test

# Pre-push checks (comprehensive)
pre-push: check-all

# ============================================================================
# MAINTENANCE
# ============================================================================

# Update Rust toolchain
update-rust:
    rustup update

# Update all tools
update-tools:
    @if command -v cargo-install-update >/dev/null 2>&1; then \
        cargo install-update -a; \
    else \
        echo "❌ cargo-update not installed. Install with: cargo install cargo-update"; \
    fi

# Clean everything including caches
clean-all: clean
    @echo "🧹 Deep cleaning..."
    cargo clean
    rm -rf node_modules/
    rm -rf {{tools_dir}}/
    @echo "✅ Deep cleaned!"

# Check for outdated dependencies
outdated:
    @if command -v cargo-outdated >/dev/null 2>&1; then \
        cargo outdated; \
    else \
        echo "❌ cargo-outdated not installed. Install with: cargo install cargo-outdated"; \
    fi

# ============================================================================
# HELP & DOCUMENTATION
# ============================================================================

# Show quick start guide
quickstart:
    @echo "🚀 Quick Start Guide"
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo "Frontend Setup:"
    @echo "  1. just check-installed     - Check what's installed"
    @echo "  2. just setup               - Install all dependencies"
    @echo "  3. just dev                 - Start development server"
    @echo ""
    @echo "Database Setup (Phase 3 - RSVP System):"
    @echo "  4. just install-supabase-cli - Install Supabase CLI"
    @echo "  5. just db-login             - Login to Supabase (one time)"
    @echo "  6. just db-init              - Set up schema + test data"
    @echo "  7. Visit /rsvp and test with code: TEST123"
    @echo ""
    @echo "That's it! You're ready to go! 🎉"
    @echo ""
    @echo "Database commands:"
    @echo "  just db-setup   - Run schema migration only"
    @echo "  just db-seed    - Add test data only"
    @echo "  just db-check   - Test database connection"
    @echo "  just db-info    - Show database information"
    @echo ""
    @echo "See README.md and docs/PHASE3_SETUP.md for full documentation"

# Show common commands
help:
    @echo "🎯 Common Commands"
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo "Setup:"
    @echo "  just check-installed  - Check installed tools"
    @echo "  just setup            - Install everything (run once)"
    @echo "  just install-trunk    - Install Trunk only"
    @echo "  just install-tools    - Install optional dev tools"
    @echo "  just install-supabase-cli - Install Supabase CLI"
    @echo ""
    @echo "Development:"
    @echo "  just dev              - Start dev server (handles deps)"
    @echo "  just build            - Build project"
    @echo "  just test             - Run tests"
    @echo ""
    @echo "Database (Phase 3 - Requires Supabase CLI):"
    @echo "  just db-login         - Login to Supabase (one time)"
    @echo "  just db-init          - Set up database (schema + test data)"
    @echo "  just db-setup         - Run schema migration only"
    @echo "  just db-seed          - Add test data only"
    @echo "  just db-check         - Test database connection"
    @echo "  just db-info          - Show schema information"
    @echo "  just db-dashboard     - Open Supabase dashboard"
    @echo ""
    @echo "Code Quality:"
    @echo "  just lint             - Run all linters"
    @echo "  just fix              - Auto-fix issues"
    @echo "  just audit            - Check security"
    @echo ""
    @echo "Deployment:"
    @echo "  just build-release    - Production build"
    @echo "  just deploy-build     - Build for GitHub Pages"
    @echo ""
    @echo "Run 'just --list' for all commands"
