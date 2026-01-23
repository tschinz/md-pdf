##################################################
# Variables
#

rust_env := "rustup show"
rust_edition := "2021"
open := if os() == "linux" { "xdg-open" } else if os() == "macos" { "open" } else { "start \"\" /max" }
app_name := "md-pdf"
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
    #!/usr/bin/env sh
    if [ -z "{{ file }}" ]; then
        echo "Usage: just watch <markdown-file>"
        echo "       just w <markdown-file>      # short alias"
        echo "Example: just watch examples/comprehensive-guide.md"
        echo "         just w working-test.md"
        exit 1
    fi
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
    echo "Generating comprehensive-guide PDFs with all available templates..."
    cargo run -- -t none -o examples/comprehensive-guide-none.pdf examples/comprehensive-guide.md
    cargo run -- -t simple -o examples/comprehensive-guide-simple.pdf examples/comprehensive-guide.md
    echo "✓ Created all PDFs from examples"
    echo "Files generated:"
    echo "  - examples/comprehensive-guide-none.pdf"
    echo "  - examples/comprehensive-guide-simple.pdf"
    {{ open }} examples/comprehensive-guide-none.pdf
    {{ open }} examples/comprehensive-guide-simple.pdf

# Test configuration system with Star Trek example
test-config:
    echo "Testing configuration-based template system..."
    cd test-config
    cargo run -- ./../examples/comprehensive-guide.md -o comprehensive-guide-config-test.pdf
    cargo run -- ./../examples/comprehensive-guide.md -t none -o comprehensive-guide-override-none.pdf
    echo "✓ Configuration system working correctly"
    {{ open }} comprehensive-guide-config-test.pdf
    {{ open }} comprehensive-guide-override-none.pdfs

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

# Show help for the compiled binary
help:
    cargo run -- --help
