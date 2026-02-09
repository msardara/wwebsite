# Wedding Website 💍

A beautiful, multilingual wedding website built with Leptos (Rust/WASM) for celebrating our special day in Sardinia and Tunisia.

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

### Install Trunk

```bash
cargo install trunk
```

### Install TailwindCSS

```bash
npm install -D tailwindcss
npx tailwindcss init
```

### Install Just (Recommended)

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

See `CI_README.md` for detailed tooling documentation and `CHEATSHEET.md` for quick reference.

### Quick Setup with Just

```bash
# Install all dependencies and tools
just setup
just install-tools

# Start developing
just build-css
just dev
```

### Without Just

If you prefer not to use Just, you can use cargo commands directly. See the `justfile` for the exact commands.

## Project Setup

### 1. Clone the Repository

```bash
git clone <your-repo-url>
cd wwebsite
```

### 2. Install Dependencies

```bash
cargo build
```

### 3. Set Up Supabase

#### Create a Supabase Project

1. Go to [supabase.com](https://supabase.com)
2. Create a new project
3. Wait for the database to be provisioned

#### Run Database Migrations

Execute the following SQL in the Supabase SQL Editor:

```sql
-- Create guests table
CREATE TABLE guests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  email TEXT,
  invitation_code TEXT UNIQUE NOT NULL,
  party_size INTEGER NOT NULL DEFAULT 1,
  location TEXT NOT NULL CHECK (location IN ('sardinia', 'tunisia', 'both')),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create rsvps table
CREATE TABLE rsvps (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  guest_id UUID NOT NULL REFERENCES guests(id) ON DELETE CASCADE,
  attending BOOLEAN NOT NULL,
  number_of_guests INTEGER NOT NULL,
  dietary_vegetarian BOOLEAN DEFAULT FALSE,
  dietary_vegan BOOLEAN DEFAULT FALSE,
  dietary_other TEXT,
  additional_notes TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  UNIQUE(guest_id)
);

-- Create content table
CREATE TABLE content (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  key TEXT NOT NULL,
  language TEXT NOT NULL CHECK (language IN ('en', 'fr', 'it')),
  location TEXT CHECK (location IN ('sardinia', 'tunisia', 'both', NULL)),
  value TEXT NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  UNIQUE(key, language, location)
);

-- Create photos table
CREATE TABLE photos (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  filename TEXT NOT NULL,
  caption TEXT,
  display_order INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create config table
CREATE TABLE config (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Enable Row Level Security
ALTER TABLE guests ENABLE ROW LEVEL SECURITY;
ALTER TABLE rsvps ENABLE ROW LEVEL SECURITY;
ALTER TABLE content ENABLE ROW LEVEL SECURITY;
ALTER TABLE photos ENABLE ROW LEVEL SECURITY;
ALTER TABLE config ENABLE ROW LEVEL SECURITY;

-- RLS Policies for guests (public read, authenticated write)
CREATE POLICY "Guests are viewable by everyone" ON guests
  FOR SELECT USING (true);

CREATE POLICY "Guests are insertable by authenticated users" ON guests
  FOR INSERT WITH CHECK (auth.role() = 'authenticated');

CREATE POLICY "Guests are updatable by authenticated users" ON guests
  FOR UPDATE USING (auth.role() = 'authenticated');

CREATE POLICY "Guests are deletable by authenticated users" ON guests
  FOR DELETE USING (auth.role() = 'authenticated');

-- RLS Policies for rsvps
CREATE POLICY "RSVPs are viewable by everyone" ON rsvps
  FOR SELECT USING (true);

CREATE POLICY "RSVPs are insertable by everyone" ON rsvps
  FOR INSERT WITH CHECK (true);

CREATE POLICY "RSVPs are updatable by everyone" ON rsvps
  FOR UPDATE USING (true);

-- RLS Policies for content
CREATE POLICY "Content is viewable by everyone" ON content
  FOR SELECT USING (true);

CREATE POLICY "Content is updatable by authenticated users" ON content
  FOR ALL USING (auth.role() = 'authenticated');

-- RLS Policies for photos
CREATE POLICY "Photos are viewable by everyone" ON photos
  FOR SELECT USING (true);

CREATE POLICY "Photos are manageable by authenticated users" ON photos
  FOR ALL USING (auth.role() = 'authenticated');

-- RLS Policies for config
CREATE POLICY "Config is viewable by everyone" ON config
  FOR SELECT USING (true);

CREATE POLICY "Config is updatable by authenticated users" ON config
  FOR ALL USING (auth.role() = 'authenticated');
```

#### Configure Environment Variables

Create a `.env` file in the project root:

```env
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_PUBLISHABLE_KEY=your-publishable-key
```

Get these values from your Supabase project settings:
- Go to Settings > API
- Copy the Project URL and publishable key

### 4. Build TailwindCSS

```bash
npx tailwindcss -i ./style/main.css -o ./style/output.css --watch
```

(Keep this running in a separate terminal)

## Development

### Run Development Server

```bash
trunk serve --open
```

This will:
- Build the WASM application
- Start a local development server at `http://127.0.0.1:3000`
- Watch for file changes and rebuild automatically

### Development Tips

- Hot reload is enabled by default
- Check browser console for any WASM errors
- Use `console_error_panic_hook` for better error messages

## Building for Production

### Build the Project

```bash
trunk build --release
```

This creates optimized WASM files in the `dist/` directory.

## Deployment to GitHub Pages

### Option 1: Manual Deployment

1. Build the project:
   ```bash
   trunk build --release
   ```

2. Push the `dist/` folder to the `gh-pages` branch:
   ```bash
   git subtree push --prefix dist origin gh-pages
   ```

3. Enable GitHub Pages in your repository settings:
   - Go to Settings > Pages
   - Source: Deploy from branch
   - Branch: gh-pages / root

### Option 2: GitHub Actions (Recommended)

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]

jobs:
  build-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Install Trunk
        run: cargo install trunk
        
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          
      - name: Install TailwindCSS
        run: npm install -D tailwindcss
        
      - name: Build TailwindCSS
        run: npx tailwindcss -i ./style/main.css -o ./style/output.css
        
      - name: Build with Trunk
        run: trunk build --release
        
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```

## Project Structure

```
wwebsite/
├── src/
│   ├── components.rs    # Components module (modern Rust 2018+ style)
│   ├── components/      # Reusable UI components
│   │   └── layout.rs    # Main layout with navigation
│   ├── pages.rs         # Pages module declaration
│   ├── pages/           # Page components
│   │   ├── home.rs      # Home page
│   │   ├── events.rs    # Events information
│   │   ├── gallery.rs   # Photo gallery
│   │   ├── rsvp.rs      # RSVP form
│   │   └── admin.rs     # Admin dashboard
│   ├── services.rs      # Services module declaration
│   ├── services/        # Backend services
│   │   └── supabase.rs  # Supabase client
│   ├── types.rs         # Data types and models
│   ├── i18n.rs          # Internationalization
│   └── lib.rs           # Library entry point
├── style/
│   ├── main.css         # Main CSS with Tailwind (source)
│   └── output.css       # Compiled Tailwind CSS
├── public/              # Static assets
│   └── photos/          # Photo storage
├── index.html           # HTML entry point
├── Trunk.toml           # Trunk configuration
├── Cargo.toml           # Rust dependencies
├── tailwind.config.js   # TailwindCSS configuration
├── justfile             # Task automation
├── docs/
│   └── MODULE_STRUCTURE.md  # Module organization guide
├── REQUIREMENTS.md      # Project requirements
└── README.md            # This file
```

**Note**: This project uses the modern Rust 2018+ module system (no `mod.rs` files). 
See `docs/MODULE_STRUCTURE.md` for details.

## Customization

### Changing Colors

Edit `tailwind.config.js` and `style/main.css` to customize the color scheme:

```css
:root {
  --color-primary: #f4c2c2;     /* Blush pink */
  --color-secondary: #a8c5a8;   /* Sage green */
  --color-accent: #d4af37;      /* Gold */
  --color-background: #fffaf5;  /* Ivory */
}
```

### Adding Content

Currently, content is hardcoded. In Phase 5, you'll be able to edit content through the admin panel, which will store it in Supabase.

### Managing Guests

Use the admin panel (when implemented in Phase 4) or directly insert into Supabase:

```sql
INSERT INTO guests (name, invitation_code, party_size, location)
VALUES ('John Doe', 'ABC123', 2, 'both');
```

## Development Phases

- [x] **Phase 1**: Project Setup & Infrastructure
- [x] **Phase 2**: Guest Interface Core (Home, Events, Gallery)
- [ ] **Phase 3**: RSVP System with Supabase Integration
- [ ] **Phase 4**: Admin Interface
- [ ] **Phase 5**: Content Management System
- [ ] **Phase 6**: Polish & Final Deployment

## Troubleshooting

### WASM Build Fails

```bash
# Make sure WASM target is installed
rustup target add wasm32-unknown-unknown

# Clear cache and rebuild
cargo clean
trunk clean
trunk build
```

### TailwindCSS Not Working

```bash
# Rebuild CSS
npx tailwindcss -i ./style/main.css -o ./style/output.css
```

### Supabase Connection Issues

- Check your `.env` file has correct credentials
- Verify RLS policies are set up correctly
- Check browser console for CORS errors

## Contributing

This is a personal wedding website project. If you're using it as a template:

1. Fork the repository
2. Customize for your needs
3. Update the README with your information
4. Deploy to your own GitHub Pages

## License

This project is personal and provided as-is for educational purposes.

## Support

For issues or questions:
- Check the [REQUIREMENTS.md](REQUIREMENTS.md) for detailed specifications
- Review Leptos documentation: https://leptos.dev
- Check Supabase docs: https://supabase.com/docs

---

**Made with ❤️ and 🦀 Rust**