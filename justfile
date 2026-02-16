# Wedding Website - Justfile
# Build automation for Leptos/WASM project
# Load environment variables from .env file

set dotenv-load := true

# Default recipe - show available commands
default:
    @just --list

# Tools directory for local binary installations

tools_dir := ".tools"

# ============================================================================
# SETUP & INSTALLATION
# ============================================================================

# Install all required tools and dependencies
setup: _install-rust _install-trunk _install-wasm-opt _install-tailwindcss _build-css
    @echo "Setup complete. Run 'just dev' to start developing."

# Install standalone TailwindCSS CLI binary (idempotent)
_install-tailwindcss:
    #!/usr/bin/env sh
    if [ -f {{ tools_dir }}/bin/tailwindcss ]; then
        echo "TailwindCSS CLI already installed."
        exit 0
    fi
    echo "Installing standalone TailwindCSS CLI to {{ tools_dir }}/..."
    mkdir -p {{ tools_dir }}/bin
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ARCH="arm64"
    elif [ "$ARCH" = "x86_64" ]; then
        ARCH="x64"
    else
        echo "Error: Unsupported architecture: $ARCH"
        exit 1
    fi
    if [ "$OS" = "darwin" ]; then
        PLATFORM="macos-${ARCH}"
    elif [ "$OS" = "linux" ]; then
        PLATFORM="linux-${ARCH}"
    else
        echo "Error: Unsupported OS: $OS"
        exit 1
    fi
    VERSION="v3.4.17"
    URL="https://github.com/tailwindlabs/tailwindcss/releases/download/${VERSION}/tailwindcss-${PLATFORM}"
    echo "Downloading $URL..."
    curl -L -o {{ tools_dir }}/bin/tailwindcss "$URL"
    chmod +x {{ tools_dir }}/bin/tailwindcss
    echo "TailwindCSS CLI installed successfully."

# Install Rust via rustup (idempotent)
_install-rust:
    @if command -v rustc >/dev/null 2>&1; then \
        echo "Rust already installed: $$(rustc --version)"; \
    else \
        echo "Installing Rust via rustup..."; \
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
        echo "Rust installed successfully. Please restart your shell or run: source $$HOME/.cargo/env"; \
    fi

# Install Trunk build tool (idempotent)
_install-trunk:
    #!/usr/bin/env sh
    if [ -f {{ tools_dir }}/bin/trunk ]; then
        echo "Trunk already installed: $({{ tools_dir }}/bin/trunk --version)"
        exit 0
    fi
    echo "Installing Trunk from GitHub releases to {{ tools_dir }}/..."
    mkdir -p {{ tools_dir }}/bin
    PROJECT_DIR=$(pwd)
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then
        ARCH="x86_64"
    elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ARCH="aarch64"
    else
        echo "Error: Unsupported architecture: $ARCH"
        exit 1
    fi
    if [ "$OS" = "darwin" ]; then
        OS="apple-darwin"
    elif [ "$OS" = "linux" ]; then
        OS="unknown-linux-gnu"
    else
        echo "Error: Unsupported OS: $OS"
        exit 1
    fi
    VERSION="v0.21.14"
    FILENAME="trunk-$ARCH-$OS.tar.gz"
    URL="https://github.com/trunk-rs/trunk/releases/download/$VERSION/$FILENAME"
    echo "Downloading $URL..."
    cd /tmp
    curl -L -o "$FILENAME" "$URL"
    tar xzf "$FILENAME"
    cp trunk "$PROJECT_DIR/{{ tools_dir }}/bin/"
    rm -f trunk "$FILENAME"
    cd "$PROJECT_DIR"
    echo "Trunk installed successfully: $({{ tools_dir }}/bin/trunk --version)"

# Install wasm-opt from binaryen releases (idempotent)
_install-wasm-opt:
    #!/usr/bin/env sh
    if [ -f {{ tools_dir }}/bin/wasm-opt ]; then
        echo "wasm-opt already installed: $({{ tools_dir }}/bin/wasm-opt --version)"
        exit 0
    fi
    echo "Installing wasm-opt from binaryen to {{ tools_dir }}/..."
    mkdir -p {{ tools_dir }}/bin
    PROJECT_DIR=$(pwd)
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then
        ARCH="x86_64"
    elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        ARCH="arm64"
    else
        echo "Error: Unsupported architecture: $ARCH"
        exit 1
    fi
    if [ "$OS" = "darwin" ]; then
        OS="macos"
    elif [ "$OS" = "linux" ]; then
        OS="linux"
    else
        echo "Error: Unsupported OS: $OS"
        exit 1
    fi
    VERSION="version_117"
    FILENAME="binaryen-$VERSION-$ARCH-$OS.tar.gz"
    URL="https://github.com/WebAssembly/binaryen/releases/download/$VERSION/$FILENAME"
    echo "Downloading $URL..."
    cd /tmp
    curl -L -o "$FILENAME" "$URL"
    tar xzf "$FILENAME"
    mkdir -p "$PROJECT_DIR/{{ tools_dir }}/lib"
    cp "binaryen-$VERSION/bin/wasm-opt" "$PROJECT_DIR/{{ tools_dir }}/bin/"
    if [ -d "binaryen-$VERSION/lib" ]; then
        cp -r "binaryen-$VERSION/lib/"* "$PROJECT_DIR/{{ tools_dir }}/lib/"
    fi
    rm -rf "binaryen-$VERSION" "$FILENAME"
    cd "$PROJECT_DIR"
    echo "wasm-opt installed successfully: $({{ tools_dir }}/bin/wasm-opt --version)"

# Install cargo-binstall for fast binary installations
_install-binstall:
    @if command -v cargo-binstall >/dev/null 2>&1; then \
        echo "cargo-binstall already installed."; \
    else \
        echo "Installing cargo-binstall..."; \
        cargo install cargo-binstall; \
        echo "cargo-binstall installed successfully."; \
    fi

# Install optional development tools (used by CI)
install-tools: _install-binstall
    @echo "Installing development tools..."
    @command -v cargo-audit >/dev/null 2>&1 || cargo binstall -y cargo-audit
    @command -v cargo-machete >/dev/null 2>&1 || cargo binstall -y cargo-machete
    @command -v cargo-deny >/dev/null 2>&1 || cargo binstall -y cargo-deny
    @command -v typos >/dev/null 2>&1 || cargo binstall -y typos-cli
    @command -v cargo-llvm-cov >/dev/null 2>&1 || cargo binstall -y cargo-llvm-cov
    @echo "Tools installed."

# Install Supabase CLI (idempotent)
_install-supabase-cli:
    @if command -v supabase >/dev/null 2>&1; then \
        echo "Supabase CLI already installed: $$(supabase --version)"; \
    elif command -v brew >/dev/null 2>&1; then \
        echo "Installing Supabase CLI via Homebrew..."; \
        brew install supabase/tap/supabase; \
        echo "Supabase CLI installed successfully."; \
    else \
        echo "Error: Homebrew not found."; \
        echo ""; \
        echo "Please install Supabase CLI manually:"; \
        echo "  macOS:   brew install supabase/tap/supabase"; \
        echo "  Linux:   brew install supabase/tap/supabase"; \
        echo "  Windows: scoop install supabase"; \
        echo ""; \
        echo "See: https://supabase.com/docs/guides/cli/getting-started"; \
        exit 1; \
    fi

# ============================================================================
# DEVELOPMENT
# ============================================================================

# Run development server with hot reload (opens browser)
dev: _install-trunk _build-css _require-env
    @echo "Starting development server..."
    @set -a && . ./.env && set +a && {{ tools_dir }}/bin/trunk serve --open

# Build CSS once
_build-css: _install-tailwindcss
    @if [ ! -f "style/output.css" ] || [ "style/main.css" -nt "style/output.css" ]; then \
        echo "Building CSS..."; \
        {{ tools_dir }}/bin/tailwindcss -i ./style/main.css -o ./style/output.css; \
    else \
        echo "CSS already up to date."; \
    fi

# Run dev server and CSS watch in parallel
dev-all: _install-trunk _install-tailwindcss _require-env
    #!/usr/bin/env sh
    set -a && . ./.env && set +a
    {{ tools_dir }}/bin/tailwindcss -i ./style/main.css -o ./style/output.css --watch &
    CSS_PID=$!
    {{ tools_dir }}/bin/trunk serve
    kill $CSS_PID 2>/dev/null

# ============================================================================
# BUILD
# ============================================================================

# Build project (debug mode)
build: _install-trunk _build-css _require-env
    @echo "Building project..."
    @set -a && { [ -f ./.env ] && . ./.env || true; } && set +a && {{ tools_dir }}/bin/trunk build

# Build for production (optimized)
build-release: _install-trunk _install-tailwindcss _require-env
    @echo "Building for production..."
    {{ tools_dir }}/bin/tailwindcss -i ./style/main.css -o ./style/output.css --minify
    @set -a && { [ -f ./.env ] && . ./.env || true; } && set +a && {{ tools_dir }}/bin/trunk build --release
    @just _optimize-wasm

_require-env:
    @if [ -n "${SUPABASE_URL:-}" ] && [ -n "${SUPABASE_PUBLISHABLE_KEY:-}" ]; then \
        exit 0; \
    elif [ -f .env ]; then \
        exit 0; \
    else \
        echo "Error: SUPABASE_URL and SUPABASE_PUBLISHABLE_KEY are not set and .env file not found."; \
        echo "Run 'just db-configure' to set up environment variables."; \
        exit 1; \
    fi

# Optimize WASM files with wasm-opt
_optimize-wasm: _install-wasm-opt
    #!/usr/bin/env sh
    echo "Optimizing WASM with wasm-opt..."
    if [ -f {{ tools_dir }}/bin/wasm-opt ]; then
        WASM_OPT="{{ tools_dir }}/bin/wasm-opt"
    elif command -v wasm-opt >/dev/null 2>&1; then
        WASM_OPT="wasm-opt"
    else
        echo "Error: wasm-opt not found"
        exit 1
    fi
    for wasm in dist/*.wasm; do
        if [ -f "$wasm" ]; then
            echo "Optimizing $wasm..."
            $WASM_OPT -Oz "$wasm" -o "$wasm.tmp" && mv "$wasm.tmp" "$wasm"
        fi
    done
    echo "WASM optimization complete."

# ============================================================================
# CODE QUALITY
# ============================================================================

# Format code
fmt:
    @echo "Formatting code..."
    cargo fmt

# Check if code is formatted
fmt-check:
    @echo "Checking code format..."
    cargo fmt -- --check

# Run clippy linter
clippy:
    @echo "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

# Check for typos in code and docs
typos:
    @echo "Checking for typos..."
    @typos

# Run all linting checks
lint: fmt-check clippy typos

# ============================================================================
# TESTING
# ============================================================================

# Run all tests
test:
    @echo "Running tests..."
    SUPABASE_URL=https://dummy.supabase.co SUPABASE_PUBLISHABLE_KEY=dummy-key cargo test --all-features

# Run tests with coverage (used by CI)
test-coverage:
    @echo "Running tests with coverage..."
    SUPABASE_URL=https://dummy.supabase.co SUPABASE_PUBLISHABLE_KEY=dummy-key cargo llvm-cov --lcov --output-path lcov.info --all-features

# ============================================================================
# SECURITY & AUDITING
# ============================================================================

# Check for security vulnerabilities
audit:
    @echo "Checking for vulnerabilities..."
    cargo audit

# Check for unused dependencies
unused:
    @echo "Checking for unused dependencies..."
    cargo machete

# Check licenses of dependencies
licenses:
    @echo "Checking licenses..."
    cargo deny check licenses

# Run all dependency checks
check-deps: audit unused licenses

# ============================================================================
# DATABASE (SUPABASE)
# ============================================================================

# Login to Supabase CLI (run once)
_db-login: _install-supabase-cli
    @if supabase projects list >/dev/null 2>&1; then \
        echo "Already authenticated with Supabase."; \
    else \
        echo "Logging in to Supabase..."; \
        supabase login; \
    fi

# Link to Supabase project
_db-link: _install-supabase-cli _db-login
    @if [ -f "supabase/.temp/project-ref" ]; then \
        echo "Already linked to project: $$(cat supabase/.temp/project-ref)"; \
    else \
        echo "Linking to Supabase project..."; \
        supabase link || \
            (echo ""; \
             echo "Error: Failed to link."; \
             exit 1); \
        echo "Successfully linked to project."; \
    fi

# Run database migrations
db-migrate: _install-supabase-cli
    @echo "Running database migrations..."
    @echo "Pushing migrations to database..."
    @if supabase db push --include-all; then \
        echo "Migrations complete."; \
        echo ""; \
        echo "Database updated with latest schema changes."; \
    else \
        echo ""; \
        echo "Error: Migration failed."; \
        echo ""; \
        echo "Manual migration:"; \
        echo "  1. Open: https://stmfsurqatlfqieuxclt.supabase.co"; \
        echo "  2. Go to SQL Editor"; \
        echo "  3. Run migrations from supabase/migrations/"; \
        exit 1; \
    fi

# Configure .env file automatically from Supabase CLI
db-configure: _install-supabase-cli _db-link
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Configuring environment variables from Supabase..."
    echo ""
    if [ ! -f "supabase/.temp/project-ref" ]; then
        echo "Error: Project not linked. Run: just _db-link"
        exit 1
    fi
    PROJECT_REF=$(cat supabase/.temp/project-ref)
    echo "Project: $PROJECT_REF"
    echo "Fetching API keys..."
    PUB_KEY=$(supabase projects api-keys --project-ref $PROJECT_REF 2>/dev/null | grep "sb_publishable" | head -1 | awk '{print $3}')
    if [ -z "$PUB_KEY" ]; then
        echo "Error: Failed to fetch API keys. You may need to login again: just _db-login"
        exit 1
    fi
    echo "SUPABASE_URL=https://$PROJECT_REF.supabase.co" > .env
    echo "SUPABASE_PUBLISHABLE_KEY=$PUB_KEY" >> .env
    echo ""
    echo ".env file configured."
    echo ""
    echo "Contents:"
    echo "  SUPABASE_URL=https://$PROJECT_REF.supabase.co"
    echo "  SUPABASE_PUBLISHABLE_KEY=$PUB_KEY"
    echo ""
    echo "Next: Rebuild the project with 'just build' or 'just dev'"

# Create admin user in Supabase Auth (pre-confirmed via Admin API)
# Usage: just db-create-admin email=admin@example.com password=secret

# just db-create-admin  (interactive prompts)
db-create-admin email="" password="": _install-supabase-cli _db-link
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Creating admin user..."
    echo ""
    if [ ! -f "supabase/.temp/project-ref" ]; then
        echo "Error: Project not linked. Run: just _db-link"
        exit 1
    fi
    PROJECT_REF=$(cat supabase/.temp/project-ref)
    EMAIL="{{ email }}"
    PASSWORD="{{ password }}"
    if [ -z "$EMAIL" ]; then
        printf "Email: "
        read -r EMAIL
    fi
    if [ -z "$EMAIL" ]; then
        echo "Error: Email is required."
        exit 1
    fi
    if [ -z "$PASSWORD" ]; then
        printf "Password (leave empty to generate one): "
        read -r -s PASSWORD
        echo ""
    fi
    if [ -z "$PASSWORD" ]; then
        PASSWORD=$(openssl rand -base64 16)
        echo "Generated password."
    fi
    SERVICE_KEY=$(supabase projects api-keys --project-ref "$PROJECT_REF" 2>/dev/null | grep "service_role" | head -1 | awk '{print $3}')
    if [ -z "$SERVICE_KEY" ]; then
        echo "Error: Failed to fetch service_role key."
        exit 1
    fi
    echo "Creating user with email: $EMAIL"
    echo ""
    RESPONSE=$(curl -s -X POST "https://$PROJECT_REF.supabase.co/auth/v1/admin/users" \
        -H "apikey: $SERVICE_KEY" \
        -H "Authorization: Bearer $SERVICE_KEY" \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\",\"email_confirm\":true}")
    if echo "$RESPONSE" | grep -q '"id"'; then
        echo "Admin user created and confirmed."
    elif echo "$RESPONSE" | grep -q "already been registered"; then
        echo "User already exists. Updating password..."
        USER_ID=$(curl -s "https://$PROJECT_REF.supabase.co/auth/v1/admin/users" \
            -H "apikey: $SERVICE_KEY" \
            -H "Authorization: Bearer $SERVICE_KEY" \
            | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
        if [ -n "$USER_ID" ]; then
            curl -s -X PUT "https://$PROJECT_REF.supabase.co/auth/v1/admin/users/$USER_ID" \
                -H "apikey: $SERVICE_KEY" \
                -H "Authorization: Bearer $SERVICE_KEY" \
                -H "Content-Type: application/json" \
                -d "{\"password\":\"$PASSWORD\",\"email_confirm\":true}" > /dev/null
            echo "Password updated."
        else
            echo "Warning: Could not find user ID to update."
        fi
    else
        echo "Error: Unexpected response: $RESPONSE"
    fi
    echo ""
    echo "Login credentials:"
    echo "  Email: $EMAIL"
    echo "  Password: $PASSWORD"
    echo ""
    echo "IMPORTANT: Save this password securely."

# ============================================================================
# CI
# ============================================================================

# Simulate CI pipeline locally
ci: lint test build-release
    @echo "CI pipeline complete."

# ============================================================================
# MAINTENANCE
# ============================================================================

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    rm -rf dist/
    rm -rf style/output.css
    rm -rf {{ tools_dir }}/
    @echo "Cleaned."
