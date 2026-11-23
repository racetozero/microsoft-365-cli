// Bootstraps the help tables in crates/m365-core/src/help/data/ from the
// upstream docs pages. The generated files are committed and edited by hand
// afterwards; rerun this only to pick up a new upstream release and review
// the diff.
//
//   node scripts/extract-upstream.mjs /tmp/cli-microsoft365 > target/commands.json
//   node scripts/gen-help.mjs /tmp/cli-microsoft365 target/commands.json
//
// Per command it keeps the summary, the option descriptions, remarks (as
// notes), permissions, examples (caption and command) and links. The
// `## Response` samples are dropped.
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const here = path.dirname(fileURLToPath(import.meta.url));
const [root = '/tmp/cli-microsoft365', manifestPath = path.join(here, '..', 'target', 'commands.json')] = process.argv.slice(2);
const docs = path.join(root, 'docs', 'docs');
const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));
const out = path.join(here, '..', 'crates', 'm365-core', 'src', 'help', 'data');

// ---- Markdown to plain text ------------------------------------------------------

function inline(s) {
  return s
    .replace(/<br\s*\/?>/gi, ' ')
    .replace(/<\/?[a-zA-Z][^>]*>/g, '')
    // links: keep the text; keep the address when the text says little
    .replace(/\[([^\]]+)\]\(([^)]+)\)/g, (_, t, u) => (/^https?:/.test(u) && !/^(https?:|www\.)/.test(t) && /^(here|this|link|this page|this article|documentation|docs)$/i.test(t.trim()) ? `${t} (${u})` : /^https?:/.test(u) && /^https?:/.test(t) ? u : t))
    .replace(/\*\*([^*]+)\*\*/g, '$1')
    .replace(/(^|[^\w*])\*([^*\s][^*]*?)\*(?=[^\w*]|$)/g, '$1$2')
    .replace(/(^|[^\w])__([^_]+)__(?=[^\w]|$)/g, '$1$2')
    .replace(/(^|[^\w`])_([^_\s][^_]*?)_(?=[^\w`]|$)/g, '$1$2')
    .replace(/&nbsp;/g, ' ')
    .replace(/&lt;/g, '<')
    .replace(/&gt;/g, '>')
    .replace(/&amp;/g, '&')
    .replace(/\s+/g, ' ')
    .trim();
}

// A block of Markdown as paragraphs of plain text (lists and tables kept as lines).
function paragraphs(md) {
  const out = [];
  let cur = [];
  const flush = () => {
    if (cur.length) out.push(cur.join('\n'));
    cur = [];
  };
  const lines = md.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    let line = lines[i];
    if (/^\s*```/.test(line)) {
      // a code block inside remarks: keep its lines indented
      flush();
      const code = [];
      for (i++; i < lines.length && !/^\s*```/.test(lines[i]); i++) code.push('    ' + lines[i].trimEnd());
      if (code.length) out.push(code.join('\n'));
      continue;
    }
    if (/^\s*:::/.test(line)) {
      // admonitions (:::tip, :::warning ... :::) become their own paragraphs
      flush();
      continue;
    }
    if (/^\s*<\/?(Tabs|TabItem|Global|CLISettings)\b/.test(line) || /^import /.test(line)) continue;
    if (/^\s*$/.test(line)) {
      flush();
      continue;
    }
    if (/^\s*\|?\s*:?-{3,}/.test(line) && line.includes('|')) continue; // table separator
    if (/^\s*\|/.test(line) || (line.split('|').length >= 3 && !/`[^`]*\|[^`]*`/.test(line))) {
      flush();
      out.push(line.trim().replace(/^\|\s*|\s*\|$/g, '').split(/\s*\|\s*/).map(inline).join(' | '));
      continue;
    }
    const item = line.match(/^(\s*)([-*]|\d+\.)\s+(.*)/);
    if (item) {
      flush();
      cur.push(`${item[1].length >= 2 ? '  ' : ''}- ${inline(item[3])}`);
      flush();
      continue;
    }
    const h = line.match(/^#{3,6}\s+(.*)/);
    if (h) {
      flush();
      out.push(inline(h[1]) + ':');
      continue;
    }
    if (cur.length && /^\s*- /.test(cur[cur.length - 1])) flush();
    cur.push(line);
  }
  flush();
  return out.map(p => (/^( {4}|\s*- |.* \| )/m.test(p) ? p : inline(p))).filter(p => p && p !== '-');
}

// ---- page parsing -----------------------------------------------------------------

function sections(text) {
  const body = text.replace(/^import .*$/gm, '');
  const parts = {};
  let name = '_head';
  for (const line of body.split(/\r?\n/)) {
    const m = line.match(/^##\s+(.*?)\s*$/);
    if (m && !line.startsWith('###')) {
      name = m[1].toLowerCase();
      parts[name] = parts[name] || [];
      continue;
    }
    (parts[name] = parts[name] || []).push(line);
  }
  return Object.fromEntries(Object.entries(parts).map(([k, v]) => [k, v.join('\n')]));
}

function summary(head) {
  const t = head.split(/\r?\n/);
  const i = t.findIndex(l => /^#\s/.test(l));
  const rest = t.slice(i + 1).join('\n');
  return paragraphs(rest)[0] || '';
}

// `-i, --id [id]` followed by `: description` lines
function options(md) {
  const res = [];
  if (!md) return res;
  const lines = md.split(/\r?\n/);
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(/^\s*`([^`]+)`\s*$/);
    if (!m) continue;
    const long = (m[1].match(/--([A-Za-z0-9][\w-]*)/) || m[1].match(/^([A-Za-z][\w-]*)\s+[[<]/) || [])[1];
    if (!long) continue;
    const desc = [];
    for (let j = i + 1; j < lines.length; j++) {
      const l = lines[j];
      if (/^\s*`[^`]+`\s*$/.test(l) || /^\s*```/.test(l)) break;
      if (/^:\s?/.test(l)) desc.push(l.replace(/^:\s?/, ''));
      else if (desc.length && l.trim()) desc.push(l.trim());
      else if (desc.length && !l.trim()) desc.push('');
    }
    res.push([long, paragraphs(desc.join('\n')).join('\n\n')]);
  }
  return res;
}

function permissions(md) {
  const res = [];
  const notes = [];
  if (!md) return { res, notes };
  let kind = '';
  const rest = [];
  for (const line of md.split(/\r?\n/)) {
    const tab = line.match(/<TabItem value="([^"]+)"/);
    if (tab) { kind = tab[1]; continue; }
    const row = line.trim().match(/^\|(.*)\|$/);
    if (row) {
      const cells = row[1].split('|').map(c => inline(c));
      if (/^-+$/.test(cells[0].replace(/:/g, '')) || /^resource$/i.test(cells[0])) continue;
      if (cells.length >= 2) res.push([kind, cells[0], cells.slice(1).join(', ')]);
      continue;
    }
    rest.push(line);
  }
  notes.push(...paragraphs(rest.join('\n')));
  return { res, notes };
}

function examples(md) {
  const res = [];
  if (!md) return res;
  const lines = md.split(/\r?\n/);
  let caption = [];
  for (let i = 0; i < lines.length; i++) {
    if (/^\s*```/.test(lines[i])) {
      const code = [];
      for (i++; i < lines.length && !/^\s*```/.test(lines[i]); i++) code.push(lines[i].trim());
      const cmd = code.filter(Boolean).join('\n');
      if (cmd) res.push([paragraphs(caption.join('\n')).join(' '), cmd]);
      caption = [];
      continue;
    }
    caption.push(lines[i]);
  }
  return res;
}

function links(md) {
  const res = [];
  if (!md) return res;
  for (const m of md.matchAll(/\[([^\]]+)\]\(([^)]+)\)/g)) res.push([inline(m[1]), m[2]]);
  for (const m of md.matchAll(/^\s*-\s+([^\[\n]+?):\s+(https?:\/\/\S+)\s*$/gm)) res.push([inline(m[1]), m[2]]);
  return res;
}

// the settings table of `cli config` as one line per setting
const settings = [];
if (fs.existsSync(path.join(docs, '_clisettings.mdx'))) {
  for (const line of fs.readFileSync(path.join(docs, '_clisettings.mdx'), 'utf8').split(/\r?\n/)) {
    const cells = line.split('|').map(c => c.trim());
    if (cells.length < 3 || /^-+$/.test(cells[0].replace(/:/g, '')) || /^setting name$/i.test(cells[0])) continue;
    const def = cells[2] && cells[2] !== '``' ? ` (default: ${cells[2]})` : '';
    settings.push(`- ${cells[0]}${def}: ${inline(cells[1])}`);
  }
}

function parse(cmd) {
  const file = path.join(docs, 'cmd', cmd.help);
  if (!fs.existsSync(file)) return null;
  const s = sections(fs.readFileSync(file, 'utf8'));
  const perms = permissions(s.permissions);
  const notes = [];
  for (const [k, v] of Object.entries(s)) {
    if (k === 'remarks' || k.startsWith('remarks ')) {
      if (k !== 'remarks') notes.push(inline(k.replace(/^remarks/, 'Remarks')) + ':');
      notes.push(...paragraphs(v));
    }
    if (k === 'available settings') {
      notes.push('Available settings:');
      notes.push(...settings);
    }
  }
  notes.push(...perms.notes);
  return {
    name: cmd.name,
    summary: summary(s._head) || cmd.description || '',
    options: options(s.options),
    notes,
    permissions: perms.res,
    examples: examples(s.examples || s.example),
    links: links(s['more information'])
  };
}

// Options the upstream pages leave out or misspell: (command, option) -> text
// or `=other` to take another documented option's text.
const OVERRIDES = {
  'outlook mail searchfolder add|sourceFoldersIds': '=sourceFolderIds',
  'spo site sharingpermission set|siteUrl': '=url',
  'spo group member remove|force': "Don't prompt for confirmation.",
  'spo list view field remove|force': "Don't prompt for confirmation.",
  'spo homesite set|order': 'The position of the home site among the Viva Connections home sites (a positive integer).',
  'spo list view add|customFormatter': 'JSON that formats the view (the view formatting definition).'
};

// Examples in the upstream pages that use options the commands do not have:
// (command, text, replacement), applied to that command's examples.
const EXAMPLE_FIXES = [
  ['outlook mail searchfolder add', '--sourceFolderIds', '--sourceFoldersIds'],
  ['pa app permission ensure', '--environment ', '--environmentName '],
  ['pa app permission remove', '--environment ', '--environmentName '],
  ['planner task add', '--asssigneePriority', '--assigneePriority'],
  ['purview threatassessment add', "--fileName 'test.txt' ", ''],
  ['spo file retentionlabel remove', ' --id 1', ''],
  ['spo folder add', '--parentFolder ', '--parentFolderUrl '],
  ['spo folder add', "--url '/ProjectFiles/Project-x'", "--parentFolderUrl '/ProjectFiles' --name 'Project-x'"],
  ['spo folder retentionlabel remove', ' --id 1', ''],
  ['spo list defaultvalue clear', '--field ', '--fieldName '],
  ['spo list defaultvalue remove', '--field ', '--fieldName '],
  ['spo list view field add', '--fieldTitle', '--title'],
  ['spo list view field add', '--fieldPosition', '--position'],
  ['spo list view field remove', '--fieldTitle', '--title'],
  ['spo list view field set', '--fieldTitle', '--title'],
  ['spo list view field set', '--fieldPosition', '--position'],
  ['spo listitem record undeclare', ' --id ', ' --listItemId '],
  ['spo site set', '--LockState', '--lockState'],
  ['spo tenant commandset set', '--newClientSideComponentId', '--clientSideComponentId'],
  ['spo tenant commandset set', ' --title ', ' --newTitle '],
  ['todo task set', ' --isReminderOn true', '']
];

function applyOverrides(cmd, h) {
  for (const [name, from, to] of EXAMPLE_FIXES) {
    if (name === cmd.name) h.examples = h.examples.map(([c, e]) => [c, e.split(from).join(to)]);
  }
  for (const [key, v] of Object.entries(OVERRIDES)) {
    const [name, opt] = key.split('|');
    if (name !== cmd.name) continue;
    const text = v.startsWith('=') ? (h.options.find(([l]) => l === v.slice(1)) || [])[1] : v;
    if (!text) continue;
    h.options = h.options.filter(([l]) => l !== opt && l !== v.slice(1) || !v.startsWith('=') && l !== opt);
    h.options.push([opt, text]);
  }
}

// ---- Rust output -----------------------------------------------------------------

// a Rust string literal (JSON escapes, with \u{...} instead of \uXXXX)
const lit = s => JSON.stringify(s).replace(/\\u([0-9a-fA-F]{4})/g, '\\u{$1}');
const tuple = a => `(${a.map(lit).join(', ')})`;
const slice = (items, f, indent) => (items.length ? `&[\n${items.map(x => `${indent}    ${f(x)},`).join('\n')}\n${indent}]` : '&[]');

function record(h) {
  const i = '        ';
  return `    Help {
        name: ${lit(h.name)},
        summary: ${lit(h.summary)},
        options: ${slice(h.options, tuple, i)},
        notes: ${slice(h.notes, lit, i)},
        permissions: ${slice(h.permissions, tuple, i)},
        examples: ${slice(h.examples, tuple, i)},
        links: ${slice(h.links, tuple, i)},
    },`;
}

const suites = new Map();
let missing = 0, noDesc = 0, noExamples = 0;
const undocumented = [];
for (const cmd of manifest.commands) {
  const h = parse(cmd);
  if (!h) { missing++; undocumented.push(cmd.name); continue; }
  applyOverrides(cmd, h);
  // sentences like "Use either `id`, `userName` or `email`, but not all." only
  // restate a rule that the help prints once ("Use exactly one of ...")
  const sets = [...(cmd.refines || []).map(r => r.rule && Array.isArray(r.rule.options) && /exactlyOne|atMostOne|atLeastOne/.test(r.rule.kind) ? r.rule.options : null), ...(cmd.optionSets || []).map(o => o.options)].filter(Boolean);
  h.options = h.options.map(([l, d]) => {
    const own = sets.filter(set => set.some(o => o.toLowerCase() === l.toLowerCase()));
    if (!own.length) return [l, d];
    const kept = d.split(/(?<=\.)\s+/).filter(sentence => {
      const named = [...sentence.matchAll(/`([^`]+)`/g)].map(m => m[1].replace(/^--?/, '').toLowerCase());
      const restates = /^(specify|use|pass|provide)\b/i.test(sentence.trim()) && own.some(set => set.filter(o => named.includes(o.toLowerCase())).length >= 2);
      return !restates;
    });
    return [l, kept.join(' ') || d];
  });
  // the docs spell some names differently (`--displayname`): use the manifest's
  h.options = h.options.map(([l, d]) => [cmd.options.find(o => o.long && o.long.toLowerCase() === l.toLowerCase())?.long || l, d]);
  // drop options the docs list but the command does not have (-h, --outputFile)
  h.options = h.options.filter(([l]) => cmd.options.some(o => o.long === l));
  const globals = new Set(['query', 'output', 'debug', 'verbose']);
  for (const o of cmd.options) {
    if (globals.has(o.long) || !o.long) continue;
    if (!h.options.some(([l, d]) => l === o.long && d)) { noDesc++; undocumented.push(`${cmd.name} --${o.long}`); }
  }
  if (!h.examples.length) noExamples++;
  const words = cmd.name.split(' ');
  const suite = words.length === 1 ? 'general' : words[0];
  if (!suites.has(suite)) suites.set(suite, []);
  suites.get(suite).push(h);
}

fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(out, { recursive: true });
const names = [...suites.keys()].sort();
for (const suite of names) {
  const items = suites.get(suite).sort((a, b) => (a.name < b.name ? -1 : 1));
  const title = suite === 'general' ? 'top-level commands (login, status, request, ...)' : `\`m365 ${suite} ...\``;
  fs.writeFileSync(path.join(out, `${suite}.rs`), `//! Help for the ${title}.\n\nuse super::Help;\n\npub static HELP: &[Help] = &[\n${items.map(record).join('\n')}\n];\n`);
}
const mods = names.map(n => `mod ${n === 'general' ? 'general' : n};`).join('\n');
fs.writeFileSync(path.join(out, 'mod.rs'), `//! The help text of every command, one file per suite. Edit these files to
//! change \`m365 <command> --help\`; \`scripts/gen-help.mjs\` only bootstraps
//! them from a new upstream release.

${mods}

/// The help of one command.
pub struct Help {
    /// The command name, for example \`entra user get\`.
    pub name: &'static str,
    /// One sentence that says what the command does.
    pub summary: &'static str,
    /// (long option name without dashes, description).
    pub options: &'static [(&'static str, &'static str)],
    /// Paragraphs shown under Notes.
    pub notes: &'static [&'static str],
    /// (kind such as Delegated or Application, resource, permissions).
    pub permissions: &'static [(&'static str, &'static str, &'static str)],
    /// (caption, command line).
    pub examples: &'static [(&'static str, &'static str)],
    /// (text, address).
    pub links: &'static [(&'static str, &'static str)],
}

/// Every suite's table.
pub static ALL: &[&[Help]] = &[${names.map(n => `${n}::HELP`).join(', ')}];

/// The help of a command by its name (not an alias). The index is built
/// once, as \`--help --agent\` looks up every command.
pub fn find(name: &str) -> Option<&'static Help> {
    static BY_NAME: std::sync::LazyLock<std::collections::HashMap<&str, &Help>> =
        std::sync::LazyLock::new(|| {
            let mut m = std::collections::HashMap::new();
            for h in ALL.iter().flat_map(|s| s.iter()) {
                m.entry(h.name).or_insert(h);
            }
            m
        });
    BY_NAME.get(name).copied()
}
`);
console.error(`help: ${manifest.commands.length - missing} commands in ${names.length} files; ${missing} without a page; ${noDesc} options without a description; ${noExamples} commands without examples`);
if (undocumented.length) console.error(`  ${undocumented.slice(0, 40).join('\n  ')}${undocumented.length > 40 ? `\n  ... ${undocumented.length - 40} more` : ''}`);
