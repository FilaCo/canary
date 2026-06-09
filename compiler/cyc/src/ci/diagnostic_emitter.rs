use std::{ops::Range, sync::Arc};

use annotate_snippets::{AnnotationKind, Group, Level, Renderer, Snippet, renderer::DecorStyle};
use cyc_diag::{Diagnostic, DiagnosticEmitter, DiagnosticLevel};
use cyc_ir::source::{SourceFile, Span};

use crate::ci::SourceMap;

#[derive(Debug)]
pub struct HumanReadableDiagnosticEmitter<'a> {
    source_map: &'a SourceMap,
    renderer: Renderer,
}

impl<'a> HumanReadableDiagnosticEmitter<'a> {
    pub fn new(source_map: &'a SourceMap) -> Self {
        Self {
            source_map,
            renderer: Renderer::styled().decor_style(DecorStyle::Unicode),
        }
    }

    /// Render a single diagnostic to a string (without trailing newline).
    fn render(&self, diag: &Diagnostic) -> String {
        // Collect every span the diagnostic points at, resolved into the file
        // that owns it plus a file-local byte range, grouped per file so each
        // file becomes one snippet. The primary span is listed first so its
        // file leads the report.
        let mut files: Vec<FileSnippet> = Vec::new();

        self.push_annotation(
            &mut files,
            diag.primary_span,
            AnnotationKind::Primary,
            // TODO: Consider primary span label?
            String::new(),
        );

        for label in &diag.labels {
            self.push_annotation(
                &mut files,
                label.span,
                AnnotationKind::Context,
                label.msg.clone(),
            );
        }

        let mut group = Group::with_title(level_of(diag.lvl).primary_title(&diag.msg));

        for file in &files {
            let mut snippet = Snippet::source(&file.source.contents)
                .path(file.source.path.to_string_lossy())
                .line_start(1);

            for ann in &file.annotations {
                snippet = snippet.annotation(ann.kind.span(ann.range.clone()).label(&ann.label));
            }

            group = group.element(snippet);
        }

        self.renderer.render(&[group]).to_string()
    }

    fn push_annotation(
        &self,
        files: &mut Vec<FileSnippet>,
        span: Span,
        kind: AnnotationKind,
        label: String,
    ) {
        let Some((source, range)) = self.resolve(span) else {
            return;
        };

        let anno = Annotation { kind, range, label };

        // Files are keyed by their global start offset; the count is tiny
        // (one diagnostic rarely spans more than a couple files), so a linear
        // scan beats a map.
        if let Some(existing) = files.iter_mut().find(|f| f.lo == source.span.lo.to_u32()) {
            existing.annotations.push(anno);
        } else {
            files.push(FileSnippet {
                lo: source.span.lo.to_u32(),
                annotations: vec![anno],
                source,
            });
        }
    }

    /// Map a global span onto `(owning file, file-local byte range)`.
    fn resolve(&self, span: Span) -> Option<(Arc<SourceFile>, Range<usize>)> {
        let source = self.source_map.get_by_pos(span.lo)?;
        let base = source.span.lo;
        let lo = (span.lo - base).to_usize();
        let hi = (span.hi - base).to_usize();
        Some((source, lo..hi))
    }
}

impl DiagnosticEmitter for HumanReadableDiagnosticEmitter<'_> {
    fn emit(&self, diag: &Diagnostic) {
        eprintln!("{}", self.render(diag));
    }
}

#[derive(Debug)]
struct FileSnippet {
    /// Global start offset of the file, used to dedup spans within one file.
    lo: u32,
    source: Arc<SourceFile>,
    annotations: Vec<Annotation>,
}

#[derive(Debug)]
struct Annotation {
    kind: AnnotationKind,
    range: Range<usize>,
    label: String,
}

fn level_of(lvl: DiagnosticLevel) -> Level<'static> {
    match lvl {
        DiagnosticLevel::Error => Level::ERROR,
        DiagnosticLevel::Warning => Level::WARNING,
        DiagnosticLevel::Note => Level::NOTE,
    }
}
