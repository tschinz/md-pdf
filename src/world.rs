//! # Embedded Typst Compiler
//!
//! This module compiles Typst templates to PDF in-process, so md-pdf does not
//! need a separate Typst installation. It builds on `typst-kit`, the toolkit the
//! Typst CLI itself uses: fonts are searched once and loaded lazily, `@preview`
//! packages are downloaded on first use into the shared Typst package cache, and
//! diagnostics are rendered the same way as by the CLI.

use std::any::Any;
use std::collections::HashMap;
use std::io::{self, IsTerminal, Read};
use std::path::Path;
use std::sync::OnceLock;

use typst::diag::{FileResult, SourceDiagnostic, Warned};
use typst::foundations::{Bytes, Datetime, Dict, Duration, Value};
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst_kit::datetime::Time;
use typst_kit::diagnostics::termcolor::Buffer;
use typst_kit::diagnostics::{self, DiagnosticFormat, DiagnosticWorld};
use typst_kit::downloader::Downloader;
use typst_kit::files::{FileLoader, FileStore, FsRoot, SystemFiles};
use typst_kit::fonts::{self, FontStore};
use typst_kit::packages::SystemPackages;
use typst_layout::PagedDocument;
use typst_pdf::PdfOptions;

/// Compiles a Typst template to PDF bytes with the embedded compiler.
///
/// The template is served at the root of the compiler's file system under
/// `template_file`, so relative paths it uses (e.g. `read(filepath)`) resolve
/// inside `root`. The markdown body (front matter already stripped) is served in
/// memory under `md_name`, taking precedence over the file of the same name on
/// disk. `@preview` packages are downloaded on first use and cached.
///
/// # Arguments
///
/// * `template_file` - File name of the template, used in diagnostics (e.g. `simple.typ`)
/// * `template` - Typst template source
/// * `root` - Directory that relative paths resolve against (the markdown file's directory)
/// * `md_name` - File name under which the template reads the markdown body
/// * `markdown` - Markdown body without front matter
/// * `inputs` - Key/value pairs exposed to the template as `sys.inputs`
///
/// # Errors
///
/// Returns the rendered compiler diagnostics if compilation or PDF export fails.
pub fn compile_pdf(
  template_file: &str,
  template: &str,
  root: &Path,
  md_name: &str,
  markdown: &str,
  inputs: Vec<(String, String)>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  let main = project_file(template_file)?;
  let memory = HashMap::from([
    (main, Bytes::new(template.as_bytes().to_vec())),
    (project_file(md_name)?, Bytes::new(markdown.as_bytes().to_vec())),
  ]);
  let inputs: Dict = inputs.into_iter().map(|(key, value)| (key.into(), Value::Str(value.into()))).collect();

  let world = DocumentWorld {
    library: LazyHash::new(Library::builder().with_inputs(inputs).build()),
    main,
    files: FileStore::new(Files {
      memory,
      system: SystemFiles::new(FsRoot::new(root.to_path_buf()), SystemPackages::new(UreqDownloader::new())),
    }),
    time: Time::system(),
  };

  let Warned { output, warnings } = typst::compile::<PagedDocument>(&world);
  // Bound the compiler's memoization cache across repeated runs (watch mode)
  comemo::evict(10);
  if !warnings.is_empty() {
    eprint!("{}", render_diagnostics(&world, &warnings));
  }
  let document = output.map_err(|errors| render_diagnostics(&world, &errors))?;
  let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).map_err(|errors| render_diagnostics(&world, &errors))?;
  Ok(pdf)
}

/// Fonts are searched once per process and shared across compilations, so
/// watch-mode rebuilds don't rescan the system. Each font is loaded on first use.
fn font_store() -> &'static FontStore {
  static FONTS: OnceLock<FontStore> = OnceLock::new();
  FONTS.get_or_init(|| {
    let mut store = FontStore::new();
    store.extend(fonts::system());
    // Typst's bundled fonts as a fallback when a template's fonts are missing
    store.extend(fonts::embedded());
    store
  })
}

/// Returns the ID of a file at the root of the project.
fn project_file(name: &str) -> Result<FileId, Box<dyn std::error::Error>> {
  let path = VirtualPath::new(name).map_err(|error| format!("invalid file name '{name}': {error:?}"))?;
  Ok(FileId::new(RootedPath::new(VirtualRoot::Project, path)))
}

/// Renders diagnostics like the Typst CLI, colored when stderr is a terminal.
fn render_diagnostics(world: &DocumentWorld, diagnostics: &[SourceDiagnostic]) -> String {
  let mut buffer = if io::stderr().is_terminal() { Buffer::ansi() } else { Buffer::no_color() };
  // Writing into an in-memory buffer does not fail
  let _ = diagnostics::emit(&mut buffer, world, diagnostics, DiagnosticFormat::Human);
  String::from_utf8_lossy(buffer.as_slice()).into_owned()
}

/// The compilation environment for one document.
struct DocumentWorld {
  library: LazyHash<Library>,
  main: FileId,
  files: FileStore<Files>,
  time: Time,
}

impl World for DocumentWorld {
  fn library(&self) -> &LazyHash<Library> {
    &self.library
  }

  fn book(&self) -> &LazyHash<FontBook> {
    font_store().book()
  }

  fn main(&self) -> FileId {
    self.main
  }

  fn source(&self, id: FileId) -> FileResult<Source> {
    self.files.source(id)
  }

  fn file(&self, id: FileId) -> FileResult<Bytes> {
    self.files.file(id)
  }

  fn font(&self, index: usize) -> Option<Font> {
    font_store().font(index)
  }

  fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
    self.time.today(offset)
  }
}

impl DiagnosticWorld for DocumentWorld {
  fn name(&self, id: FileId) -> String {
    let path = id.vpath().get_without_slash();
    match id.root() {
      VirtualRoot::Package(package) => format!("{package}/{path}"),
      VirtualRoot::Project => path.to_string(),
    }
  }
}

/// Serves the in-memory template and markdown body ahead of the file system
/// and the package storage.
struct Files {
  memory: HashMap<FileId, Bytes>,
  system: SystemFiles,
}

impl FileLoader for Files {
  fn load(&self, id: FileId) -> FileResult<Bytes> {
    match self.memory.get(&id) {
      Some(bytes) => Ok(bytes.clone()),
      None => self.system.load(id),
    }
  }
}

/// Downloads Typst packages with the same `ureq` client used by the link checker.
struct UreqDownloader(ureq::Agent);

impl UreqDownloader {
  fn new() -> Self {
    let agent = ureq::Agent::config_builder().user_agent(concat!("md-pdf/", env!("CARGO_PKG_VERSION"))).build();
    Self(agent.into())
  }
}

impl Downloader for UreqDownloader {
  fn stream(&self, key: &dyn Any, url: &str) -> io::Result<(Option<usize>, Box<dyn Read>)> {
    // Packages are downloaded on first use only, so announce it like the Typst CLI
    if let Some(package) = key.downcast_ref::<PackageSpec>() {
      eprintln!("downloading {package}");
    }
    let response = self.0.get(url).call().map_err(|error| match error {
      ureq::Error::StatusCode(404) => io::Error::new(io::ErrorKind::NotFound, error),
      error => io::Error::other(error),
    })?;
    let length = response.body().content_length().and_then(|length| usize::try_from(length).ok());
    Ok((length, Box::new(response.into_body().into_reader())))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  fn inputs(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect()
  }

  #[test]
  fn test_compile_pdf_serves_markdown_and_inputs() {
    // The file on disk still has front matter; the stripped body must win.
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("doc.md"), "---\ntitle: T\n---\n# Body").unwrap();
    let template = r##"
#let body = read(sys.inputs.filepath)
#assert(body.starts-with("# Body"), message: "front matter was not stripped")
#assert.eq(sys.inputs.fm_title, "T")
#body
"##;

    let pdf = compile_pdf(
      "test.typ",
      template,
      root.path(),
      "doc.md",
      "# Body",
      inputs(&[("filepath", "doc.md"), ("fm_title", "T")]),
    )
    .unwrap();

    assert!(pdf.starts_with(b"%PDF"));
  }

  #[test]
  fn test_compile_pdf_reports_located_diagnostics() {
    let root = tempfile::tempdir().unwrap();
    let error = compile_pdf("test.typ", "#image(\"missing.png\")", root.path(), "doc.md", "", Vec::new())
      .unwrap_err()
      .to_string();

    // Diagnostics may be colored when the test runs in a terminal
    let error = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap().replace_all(&error, "");
    assert!(error.contains("file not found"), "{error}");
    assert!(error.contains("test.typ:1:"), "{error}");
  }
}
