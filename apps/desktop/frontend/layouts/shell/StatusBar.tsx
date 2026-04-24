import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { cn } from '@/lib/utils';
import { useEffect, useState, useMemo } from 'react';
import { WorkspaceService } from '@/features/workspace/services/workspaceService';
import { useLayoutMode } from '@/hooks/useLayoutMode';

interface StatusBarProps {
  className?: string;
}

/**
 * Simple language detection by file extension.
 * Used in the status bar to show the language mode without an extra bridge call.
 */
function detectLanguageExtension(path: string): string | undefined {
  const ext = path.split('.').pop()?.toLowerCase() ?? '';
  const known: Record<string, string> = {
    rs: 'Rust',
    ts: 'TypeScript',
    tsx: 'TypeScript JSX',
    js: 'JavaScript',
    jsx: 'JavaScript JSX',
    mjs: 'JavaScript',
    cjs: 'JavaScript',
    py: 'Python',
    go: 'Go',
    toml: 'TOML',
    yaml: 'YAML',
    yml: 'YAML',
    json: 'JSON',
    jsonc: 'JSON with Comments',
    md: 'Markdown',
    mdx: 'MDX',
    html: 'HTML',
    htm: 'HTML',
    css: 'CSS',
    scss: 'SCSS',
    sass: 'SASS',
    less: 'Less',
    styl: 'Stylus',
    stylum: 'Stylus',
    vue: 'Vue',
    svelte: 'Svelte',
    c: 'C',
    h: 'C Header',
    cpp: 'C++',
    cxx: 'C++',
    cc: 'C++',
    hpp: 'C++ Header',
    hxx: 'C++ Header',
    hh: 'C++ Header',
    java: 'Java',
    class: 'Java',
    jar: 'Java Archive',
    kt: 'Kotlin',
    kts: 'Kotlin Script',
    swift: 'Swift',
    lua: 'Lua',
    rb: 'Ruby',
    php: 'PHP',
    pl: 'Perl',
    pm: 'Perl Module',
    t: 'Perl Test',
    pod: 'Perl POD',
    sh: 'Shell Script',
    bash: 'Shell Script',
    zsh: 'Shell Script',
    fish: 'Fish Shell',
    ps1: 'PowerShell',
    psm1: 'PowerShell Module',
    bat: 'Batch',
    cmd: 'Batch',
    vbs: 'VBScript',
    ahk: 'AutoHotkey',
    au3: 'AutoIt',
    asm: 'Assembly',
    s: 'Assembly',
    inc: 'Assembly Include',
    ada: 'Ada',
    adb: 'Ada Body',
    ads: 'Ada Spec',
    apl: 'APL',
    app: 'AppleScript',
    applescript: 'AppleScript',
    as: 'ActionScript',
    asc: 'AsciiDoc',
    adoc: 'AsciiDoc',
    asciidoc: 'AsciiDoc',
    asp: 'ASP',
    aspx: 'ASP.NET',
    awk: 'Awk',
    bicep: 'Bicep',
    bzl: 'Bazel',
    BUILD: 'Bazel',
    cbl: 'COBOL',
    cpy: 'COBOL',
    cob: 'COBOL',
    clj: 'Clojure',
    cljs: 'ClojureScript',
    cljc: 'Clojure Common',
    edn: 'EDN',
    coffee: 'CoffeeScript',
    cr: 'Crystal',
    dart: 'Dart',
    d: 'D',
    dockerfile: 'Dockerfile',
    dockerignore: 'Docker Ignore',
    el: 'Emacs Lisp',
    elx: 'Elixir',
    ex: 'Elixir',
    exs: 'Elixir Script',
    eex: 'Elixir Embedded',
    heex: 'HEEx',
    elm: 'Elm',
    erl: 'Erlang',
    hrl: 'Erlang Header',
    f: 'Fortran',
    f77: 'Fortran',
    f90: 'Fortran',
    f95: 'Fortran',
    f03: 'Fortran',
    f08: 'Fortran',
    for: 'Fortran',
    fsi: 'F# Script',
    fs: 'F#',
    fsx: 'F# Script',
    glsl: 'GLSL',
    frag: 'GLSL Fragment',
    vert: 'GLSL Vertex',
    graphql: 'GraphQL',
    gql: 'GraphQL',
    groovy: 'Groovy',
    gvy: 'Groovy',
    gsh: 'Groovy',
    hs: 'Haskell',
    lhs: 'Literate Haskell',
    haxe: 'Haxe',
    hx: 'Haxe',
    hxsl: 'Haxe',
    hy: 'Hy',
    idr: 'Idris',
    lidr: 'Idris',
    ini: 'INI',
    cfg: 'INI',
    conf: 'INI',
    julia: 'Julia',
    jl: 'Julia',
    latex: 'LaTeX',
    tex: 'LaTeX',
    cls: 'LaTeX Class',
    sty: 'LaTeX Style',
    bib: 'BibTeX',
    lisp: 'Lisp',
    lsp: 'Lisp',
    l: 'Lisp',
    ls: 'LiveScript',
    makefile: 'Makefile',
    mak: 'Makefile',
    mk: 'Makefile',
    matlab: 'MATLAB',
    m: 'Objective-C',
    mm: 'Objective-C++',
    ml: 'OCaml',
    mli: 'OCaml Interface',
    mll: 'OCaml Lex',
    mly: 'OCaml Yacc',
    nix: 'Nix',
    nim: 'Nim',
    nims: 'Nim Script',
    pas: 'Pascal',
    pp: 'Pascal',
    prg: 'Pascal',
    prolog: 'Prolog',
    proto: 'Protocol Buffers',
    r: 'R',
    rmd: 'R Markdown',
    racket: 'Racket',
    rkt: 'Racket',
    res: 'Reason',
    resi: 'Reason Interface',
    re: 'ReScript',
    rei: 'ReScript Interface',
    sml: 'Standard ML',
    scala: 'Scala',
    sc: 'Scala Script',
    scheme: 'Scheme',
    scm: 'Scheme',
    ss: 'Scheme',
    sql: 'SQL',
    tcl: 'Tcl',
    tf: 'Terraform',
    tfvars: 'Terraform Variables',
    tfstate: 'Terraform State',
    v: 'V',
    verilog: 'Verilog',
    vhd: 'VHDL',
    vhdl: 'VHDL',
    waaa: 'WebAssembly (Text)',
    wat: 'WebAssembly (Text)',
    wasm: 'WebAssembly (Binary)',
    xml: 'XML',
    xsd: 'XML Schema',
    xsl: 'XSL Transform',
    xhtml: 'XHTML',
    svg: 'SVG',
    zig: 'Zig',
    env: 'Environment Variables',
    gitignore: 'Git Ignore',
    gitattributes: 'Git Attributes',
    gitkeep: 'Git Keep',
    npmrc: 'npm Config',
    editorconfig: 'EditorConfig',
    eslintrc: 'ESLint Config',
    prettierrc: 'Prettier Config',
    postcssrc: 'PostCSS Config',
    babelrc: 'Babel Config',
    browserlistrc: 'Browserslist Config',
    stylelintrc: 'Stylelint Config',
    commitlintrc: 'Commitlint Config',
    jestrc: 'Jest Config',
    cypressrc: 'Cypress Config',
    huskyrc: 'Husky Config',
    lintstagedrc: 'lint‑staged Config',
    nvmrc: 'Node Version',
    phplintrc: 'PHPLint Config',
    phpcs: 'PHP CodeSniffer Config',
    phpmd: 'PHPMD Config',
  };
  return known[ext];
}

/** Data we need from the open file response (only metadata, not full content). */
interface FileMeta {
  largeFileMode: string;
  contentTruncated: boolean;
}

export function StatusBar({ className }: StatusBarProps) {
  const { currentWorkspace, isLoading, explorerUI } = useWorkspaceStore();
  const activeFilePath = explorerUI?.activeFilePath ?? null;
  const [fileMeta, setFileMeta] = useState<FileMeta | null>(null);

  const layoutMode = useLayoutMode();
  const isNarrow = layoutMode === 'narrow';

  // Fetch lightweight file metadata when the active file changes
  useEffect(() => {
    let cancelled = false;
    if (activeFilePath) {
      // openFile is lightweight enough; it reads the file on the Rust side
      // but only returns metadata and optionally truncated content.
      WorkspaceService.openFile({ path: activeFilePath }).then((resp) => {
        if (!cancelled) {
          setFileMeta({
            largeFileMode: resp.largeFileMode ?? 'Normal',
            contentTruncated: resp.contentTruncated ?? false,
          });
        }
      });
    } else {
      setFileMeta(null);
    }
    return () => {
      cancelled = true;
    };
  }, [activeFilePath]);

  // Derive presentation values from the active file path
  const fileName = useMemo(
    () => (activeFilePath ? activeFilePath.split(/[\\/]/).pop() ?? '—' : null),
    [activeFilePath]
  );

  const languageLabel = useMemo(
    () => (activeFilePath ? detectLanguageExtension(activeFilePath) ?? 'Plain Text' : null),
    [activeFilePath]
  );

  // Show a dedicated large‑file indicator only when the file is not normal
  const largeFileIndicator =
    fileMeta && fileMeta.largeFileMode !== 'Normal'
      ? `  ${fileMeta.largeFileMode === 'VeryLarge' ? 'Very Large' : 'Large'} File`
      : null;

  const truncationIndicator =
    fileMeta && fileMeta.contentTruncated ? '  (truncated)' : null;

  return (
    <div
      className={cn(
        'h-6 flex items-center justify-between px-3 text-[11px] font-sans leading-none',
        'text-primary border-t',
        className
      )}
      style={{ backgroundColor: 'var(--status-bar-background)', borderTop: '0.5px solid var(--color-divider-subtle)' }}
    >
      {/* ── Left side: workspace / operational state ── */}
      <div className="flex items-center space-x-3">
        <span className="font-medium">{currentWorkspace ? currentWorkspace.name : 'No workspace'}</span>
        {currentWorkspace && !isNarrow && (
          <span className="text-primary/70 font-mono text-[10px]">
            {currentWorkspace.rootPath.split('/').pop()}
          </span>
        )}
        {isLoading && (
          <span className="text-accent font-medium">
            <span className="inline-block w-1.5 h-1.5 rounded-full bg-accent animate-pulse mr-1 align-middle" />
            Loading…
          </span>
        )}
      </div>

      {/* ── Right side: file/editor metadata ── */}
      <div className="flex items-center space-x-3">
        {activeFilePath && (
          <>
            {/* File name (no icon – text is enough) */}
            <span className="font-medium max-w-[120px] truncate" title={activeFilePath}>
              {fileName}
            </span>

            {/* Language mode */}
            {!isNarrow && <span className="text-primary/70">{languageLabel}</span>}

            {/* Encoding & line endings – always useful when a file is open */}
            {!isNarrow && <span className="text-primary/70">UTF-8</span>}
            {!isNarrow && <span className="text-primary/70">LF</span>}

            {/* Large‑file / truncation indicators – only when relevant */}
            {(largeFileIndicator != null || truncationIndicator != null) && (
              <span className="text-yellow-500 font-medium">
                {largeFileIndicator ?? ''}
                {truncationIndicator ?? ''}
              </span>
            )}
          </>
        )}
      </div>
    </div>
  );
}
