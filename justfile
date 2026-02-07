##################################################
# Variables
#

rust_env := "rustup show"
rust_edition := "2024"
open := if os() == "linux" { "xdg-open" } else if os() == "macos" { "open" } else { "start \"\" /max" }
app_name := "md-pdf"
crate_name := "md_pdf"
args := ""
project_directory := justfile_directory()
release := `git describe --tags --always`
version := "0.1.0"
url := "https://github.com/tschinz/md-pdf"

##################################################
# COMMANDS
#

# List all commands
@default:
    just --list

# Information about the environment
@info:
    echo "Environment Informations\n------------------------\n"
    echo "OS   : {{ os() }}({{ arch() }})"
    echo "Open : {{ open }}"
    echo "Rust :"
    echo "`{{ rust_env }}`"
    echo "Typst CLI installed: `which typst || echo 'Not found'`"

# Check if all dependencies are available
check-deps:
    @echo "Checking dependencies..."
    @which cargo >/dev/null 2>&1 && echo "✓ cargo found" || echo "✗ cargo not found"
    @which typst >/dev/null 2>&1 && echo "✓ typst-cli found" || echo "✗ typst-cli not found - run 'just install'"

# Install dependencies
install:
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    cargo install typst-cli

# install the release version (default is the latest)
install-release release=release:
    cargo install --git {{ url }} --tag {{ release }}

# install the nightly release
install-nightly:
    cargo install --git {{ url }}

# Build and copy the release version of the program
build:
    cargo build --release
    mkdir -p bin && cp target/release/{{ app_name }} bin/

# create a release version of the program
changelog version=version:
    git cliff --tag {{ version }}

# Run the program in debug mode
run args=args:
    cargo run -- {{ args }}

# Watch a markdown file for changes and rebuild automatically
watch file="" args=args:
    # Convert .md to .pdf for opening
    pdf_file=$(echo "{{ file }}" | sed 's/\.md$/.pdf/')
    cargo run -- "{{ file }}" {{ args }}
    {{ open }} "$pdf_file"
    cargo run -- --watch "{{ file }}" {{ args }}

# Watch the guide with live preview
watch-guide args=args:
    cargo run -- examples/comprehensive-guide.md -o examples/comprehensive-guide.pdf {{ args }}
    {{ open }} examples/comprehensive-guide.pdf
    cargo run -- -w examples/comprehensive-guide.md -o examples/comprehensive-guide.pdf {{ args }}

# Watch the guide with live preview
pdf-guide args=args:
    cargo run -- examples/comprehensive-guide.md -o examples/comprehensive-guide.pdf {{ args }}
    {{ open }} examples/comprehensive-guide.pdf

# Generate guide PDFs from all example documents using both available templates
pdf-examples:
    echo "Generating starfleet PDFs with all available templates..."
    cargo run -- -t none -o examples/starfleet-none.pdf examples/starfleet.md
    cargo run -- -t simple -o examples/starfleet-simple.pdf examples/starfleet.md
    cargo run -- -t playful -o examples/starfleet-playful.pdf examples/starfleet.md
    cargo run -- -t brutalist -o examples/starfleet-brutalist.pdf examples/starfleet.md
    cargo run -- -t darko -o examples/starfleet-darko.pdf examples/starfleet.md
    echo "✓ Created all PDFs from examples"
    echo "Files generated:"
    echo "  - examples/starfleet-none.pdf"
    echo "  - examples/starfleet-simple.pdf"
    echo "  - examples/starfleet-playful.pdf"
    echo "  - examples/starfleet-brutalists.pdf"
    echo "  - examples/starfleet-darko.pdf"
    {{ open }} examples/starfleet-none.pdf
    {{ open }} examples/starfleet-simple.pdf
    {{ open }} examples/starfleet-playful.pdf
    {{ open }} examples/starfleet-brutalist.pdf
    {{ open }} examples/starfleet-darko.pdf

# Run tests
test:
    cargo test

# Check code with clippy
clippy:
    cargo clippy -- -D warnings

# Run rustfmt with custom configuration
rustfmt:
    find {{ invocation_directory() }} -name \*.rs -exec rustfmt --config tab_spaces=2 --edition {{ rust_edition }} {} \;

# Clean build artifacts and temporary files
clean:
    cargo clean
    rm -f *.pdf test.md

# Generate and open rustdoc documentation
doc:
    @echo "Generating rustdoc documentation..."
    cargo doc --no-deps --document-private-items
    @echo "✓ Documentation generated"
    @echo "Opening documentation in browser..."
    {{ open }} target/doc/{{ crate_name }}/index.html

# Generate rustdoc documentation without opening
doc-build:
    @echo "Generating rustdoc documentation..."
    cargo doc --no-deps --document-private-items
    @echo "✓ Documentation generated at target/doc/{{ crate_name }}/index.html"

# Generate SBOM for Dependecy Track
sbom:
    cargo sbom --output-format cyclone_dx_json_1_6 >> target/sbom-cyclone_dx_1_6.json

# Show help for the compiled binary
help:
    cargo run -- --help
