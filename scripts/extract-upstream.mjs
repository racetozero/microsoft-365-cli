// Extracts the command manifest of a built CLI for Microsoft 365 checkout.
//
//   node scripts/extract-upstream.mjs /tmp/cli-microsoft365 > target/commands.json
//   cargo run -q -p m365-codegen -- target/commands.json      # crates/m365-core/src/generated/manifest.rs
//   node scripts/gen-help.mjs /tmp/cli-microsoft365 target/commands.json   # help tables (review the diff)
//
// The checkout must have been built with `npm ci && npm run build`. The
// manifest is the single source of truth for command names, aliases,
// descriptions, options, default text-output properties, base classes and
// option schemas. `crates/m365-core/build.rs` turns it into static tables.
//
// Option schemas are recorded by walking the zod definitions, not the JSON
// schema, because the JSON schema loses custom error messages, coerced enums
// and refinements. A refinement's predicate is recorded as its JavaScript
// source (`src`) so that the Rust side can map well-known predicates such as
// `validation.isValidGuid` to native validators; its message is recorded as a
// template in which `{input}` stands for the rejected value.
import fs from 'fs';
import path from 'path';
import { pathToFileURL } from 'url';
import { init as initExpr, compile } from './js-expr.mjs';

const root = path.resolve(process.argv[2] || '/tmp/cli-microsoft365');
const dist = path.join(root, 'dist');
initExpr(root);
const { default: Command } = await import(pathToFileURL(path.join(dist, 'Command.js')));
const { cli } = await import(pathToFileURL(path.join(dist, 'cli', 'cli.js')));
const full = JSON.parse(fs.readFileSync(path.join(root, 'allCommandsFull.json'), 'utf8'));
const INPUT = '\u0000INPUT\u0000';

function baseChain(cmd) {
  const chain = [];
  let proto = Object.getPrototypeOf(cmd);
  while (proto && proto.constructor && proto.constructor.name !== 'Object') {
    chain.push(proto.constructor.name);
    proto = Object.getPrototypeOf(proto);
  }
  return chain.slice(1); // drop the concrete class
}

function isDynamic(m) {
  return m === '<dynamic>';
}

function src(fn) {
  return fn ? fn.toString().replace(/\s+/g, ' ').trim() : undefined;
}

function message(def, extra = {}) {
  if (typeof def.error !== 'function') return undefined;
  try {
    const m = def.error({ input: INPUT, code: def.check || 'custom', origin: def.origin, path: [], ...extra });
    const text = typeof m === 'string' ? m : m && typeof m.message === 'string' ? m.message : undefined;
    // a message computed from the value (path.resolve, JSON.parse) is not a template
    if (text !== undefined && (text.includes(process.cwd()) || /SyntaxError|Unexpected token|"[^"]*" is not valid JSON/.test(text))) return '<dynamic>';
    if (text !== undefined) return text.split(INPUT).join('{input}');
  } catch {
    return '<dynamic>';
  }
  return undefined;
}

// An object refinement's message may read option values from `e.input`;
// those become `{input.name}` placeholders.
function objectMessage(def) {
  if (typeof def.error !== 'function') return undefined;
  const marker = k => `\u0000P:${k}\u0000`;
  const input = new Proxy({}, { get: (_, k) => typeof k === 'string' ? marker(k) : undefined });
  try {
    const m = def.error({ input, code: 'custom', path: [] });
    const text = typeof m === 'string' ? m : m && typeof m.message === 'string' ? m.message : undefined;
    if (text !== undefined) {
      const out = text.replace(/\u0000P:([^\u0000]*)\u0000/g, '{input.$1}');
      // a message computed from the value (JSON.parse, ...) is not a template
      if (!out.includes('\u0000') && !out.includes(process.cwd())) return out;
      return '<dynamic>';
    }
  } catch { }
  return message(def);
}

function checks(def) {
  const out = [];
  for (const c of def.checks || []) {
    const cd = c._zod.def;
    const r = { k: cd.check };
    if (cd.format) r.format = cd.format;
    if (cd.pattern) r.pattern = cd.pattern.source;
    if (cd.value !== undefined) r.value = typeof cd.value === 'bigint' ? Number(cd.value) : cd.value;
    if (cd.minimum !== undefined) r.value = cd.minimum;
    if (cd.maximum !== undefined) r.value = cd.maximum;
    if (cd.inclusive !== undefined) r.inclusive = cd.inclusive;
    if (cd.check === 'custom') {
      r.src = src(cd.fn);
      if (cd.fn && cd.fn.constructor && cd.fn.constructor.name === 'AsyncFunction') r.async = true;
      if (cd.params) r.params = cd.params;
      if (cd.path) r.path = cd.path;
    }
    const m0 = message(cd);
    if (cd.check === 'custom' && !r.async && !isDynamic(m0)) { const e = compile(r.src); if (e) r.expr = e; }
    if (cd.check === 'overwrite') r.src = src(cd.tx);
    const m = message(cd);
    if (m !== undefined) r.msg = m;
    out.push(r);
  }
  return out;
}

// Flattens a zod field definition into a compact spec.
function field(schema) {
  const spec = { t: undefined, opt: false };
  const walk = (s) => {
    const def = s._zod.def;
    const cs = checks(def);
    // checks on a pipe see the transformed value, which the compiled
    // predicates (expr) do not model: those stay hooks
    if (def.type === 'pipe') for (const c of cs) delete c.expr;
    if (cs.length) spec.checks = [...(spec.checks || []), ...cs];
    if (def.alias && spec.short === undefined) spec.short = def.alias;
    if (def.autocomplete) spec.autocomplete = def.autocomplete;
    if (def.error && !['optional', 'default'].includes(def.type)) {
      const m = message(def, { code: 'invalid_type' });
      if (m !== undefined) spec.typeMsg = m;
    }
    switch (def.type) {
      case 'optional': spec.opt = true; return walk(def.innerType);
      case 'default':
      case 'prefault': spec.opt = true; spec.default = typeof def.defaultValue === 'function' ? def.defaultValue() : def.defaultValue; return walk(def.innerType);
      case 'string': {
        spec.t ??= 'string';
        if (def.format) {
          const r = { k: 'string_format', format: def.format };
          if (def.pattern) r.pattern = def.pattern.source;
          const m = message(def, { code: 'invalid_format', format: def.format });
          if (m !== undefined) r.msg = m;
          spec.checks = [r, ...(spec.checks || [])];
        }
        return;
      }
      case 'boolean': spec.t ??= 'boolean'; return;
      case 'number': spec.t ??= 'number'; if (def.coerce) spec.coerce = true; return;
      case 'enum': {
        if (spec.t === 'string' && spec.piped) spec.ci = true;
        spec.t = 'enum';
        spec.values = Object.values(def.entries);
        return;
      }
      case 'literal': spec.t = 'enum'; spec.values = def.values; return;
      case 'pipe': spec.piped = true; walk(def.in); return walk(def.out);
      case 'transform': spec.transform = src(def.transform); return;
      case 'array': {
        // `z.string().transform(v => v.split(',')).pipe(z.enum([...]).array())`:
        // the option is a string whose items are validated after the split
        if (spec.transform) {
          spec.t = 'string';
          const el = def.element._zod.def;
          if (el.type === 'enum') spec.checks = [...(spec.checks || []), { k: 'items_enum', values: Object.values(el.entries) }];
          return;
        }
        spec.t = 'array';
        return;
      }
      case 'union': spec.t = 'union'; spec.union = def.options.map(o => field(o)); return;
      case 'any': case 'unknown': spec.t ??= 'string'; return;
      default: spec.t ??= def.type; return;
    }
  };
  walk(schema);
  delete spec.piped;
  return spec;
}

// Classifies an object-level refinement into a structural rule when the
// predicate follows one of the common shapes, so the Rust side does not need
// hand-written code for it.
function classify(r) {
  let s = r.src || '';
  // `opts => { const specified = [...].filter(...).length; return specified <= 1; }`
  const blk = s.match(/^\(?(\w+)\)? => \{ const (\w+) = (\[[^\]]+\]\.filter\(\w+ => \w+ !== undefined\)\.length); return \2 (===|<=|>=|>|<) (\d); \}$/);
  if (blk) s = `${blk[1]} => ${blk[3]} ${blk[4]} ${blk[5]}`;
  const opts = r.params?.options;
  const m = s.match(/^\(?(\w+)\)? => \[([^\]]+)\]\.filter\((\w+) => \3 !== undefined\)\.length (===|<=|>=|>|<) (\d)$/);
  if (m) {
    const names = m[2].split(',').map(x => x.trim().replace(new RegExp(`^${m[1]}\\.`), ''));
    if (names.every(n => /^\w+$/.test(n))) {
      const op = m[4], n = Number(m[5]);
      const kind = op === '===' && n === 1 ? 'exactlyOne' : op === '<=' && n === 1 ? 'atMostOne' : (op === '>=' && n === 1) || (op === '>' && n === 0) ? 'atLeastOne' : op === '<' && n === 2 ? 'atMostOne' : undefined;
      if (kind) return { kind, options: names };
    }
  }
  const tm = s.match(/^\(?(\w+)\)? => \[([^\]]+)\]\.filter\(Boolean\)\.length (===|<=|>=|>) (\d)$/);
  if (tm) {
    const names = tm[2].split(',').map(x => x.trim().replace(new RegExp(`^${tm[1]}\\.`), ''));
    if (names.every(n => /^\w+$/.test(n))) {
      const op = tm[3], n = Number(tm[4]);
      const kind = op === '===' && n === 1 ? 'exactlyOneTruthy' : op === '<=' && n === 1 ? 'atMostOneTruthy' : (op === '>=' && n === 1) || (op === '>' && n === 0) ? 'atLeastOneTruthy' : undefined;
      if (kind) return { kind, options: names };
    }
  }
  const notAll = s.match(/^\(?(\w+)\)? => !\((\w+\.\w+(?: && \w+\.\w+)+)\)$/);
  if (notAll) {
    const names = notAll[2].split(' && ').map(x => x.split('.')[1]);
    return { kind: 'notAllTruthy', options: names };
  }
  const implies = s.match(/^\(?(\w+)\)? => !\1\.(\w+) \|\| (?:!!)?\1\.(\w+)$/);
  if (implies) return { kind: 'implies', options: [implies[2], implies[3]] };
  const any = s.match(/^\(?(\w+)\)? => (\(?[\w.]+ !== undefined\)?(?: \|\| \(?[\w.]+ !== undefined\)?)*)$/);
  if (any) {
    const names = [...any[2].matchAll(new RegExp(`${any[1]}\\.(\\w+) !== undefined`, 'g'))].map(x => x[1]);
    if (names.length) return { kind: 'atLeastOne', options: names };
  }
  const anyTruthy = s.match(/^\(?(\w+)\)? => ((?:!!)?\w+\.\w+(?: \|\| (?:!!)?\w+\.\w+)*)$/);
  if (anyTruthy) {
    const names = anyTruthy[2].split(' || ').map(x => x.replace(/^!!/, '').split('.')[1]);
    return { kind: 'atLeastOneTruthy', options: names };
  }
  if (opts && /Object\.values|Object\.keys/.test(s) === false) return undefined;
  return undefined;
}

function objectRefines(schema) {
  const out = [];
  let s = schema;
  // refine() on an object yields the same object type with extra checks
  for (const c of s._zod.def.checks || []) {
    const cd = c._zod.def;
    if (cd.check !== 'custom') continue;
    const r = { src: src(cd.fn) };
    const m = objectMessage(cd);
    if (m !== undefined) r.msg = m;
    if (cd.params) r.params = cd.params;
    if (cd.path) r.path = cd.path;
    if (cd.when) r.when = src(cd.when);
    const rule = classify(r);
    if (rule) r.rule = rule;
    else if (!isDynamic(r.msg)) { const e = compile(r.src); if (e) r.expr = e; }
    out.push(r);
  }
  return out;
}

const out = [];
let order = 0;
for (const info of full) {
  const mod = await import(pathToFileURL(path.join(dist, 'm365', info.file)));
  const cmd = mod.default;
  if (!(cmd instanceof Command)) throw new Error(`not a command: ${info.file}`);
  const ci = cli.getCommandInfo(cmd, info.file, info.help);
  const entry = {
    name: ci.name,
    aliases: ci.aliases || [],
    description: ci.description,
    file: info.file.replace(/\.js$/, '.ts'),
    help: info.help,
    bases: baseChain(cmd),
    order: order++,
    options: ci.options.map(o => {
      const r = { name: o.name, long: o.long };
      if (o.short) r.short = o.short;
      if (o.type) r.type = o.type;
      r.required = !!o.required;
      if (o.autocomplete) r.autocomplete = o.autocomplete.map(String);
      return r;
    }),
  };
  if (ci.defaultProperties) entry.defaultProperties = ci.defaultProperties;
  if (cmd.allowUnknownOptions()) entry.allowUnknownOptions = true;
  if (Object.prototype.hasOwnProperty.call(Object.getPrototypeOf(cmd), 'action')) entry.customAction = true;
  const outputs = cmd.allowedOutputs;
  if (outputs && outputs.join() !== 'csv,json,md,text,none') entry.allowedOutputs = outputs;
  try { if (cmd.resource) entry.resource = cmd.resource; } catch { /* abstract */ }
  try { if (cmd.usageEndpoint) entry.usageEndpoint = cmd.usageEndpoint; } catch { /* not a report */ }
  if (typeof cmd.getNamesOfOptionsWithUrls === 'function') entry.urlOptions = cmd.getNamesOfOptionsWithUrls();
  {
    // how the command translates failures: the first handler called after commandAction
    const ts = fs.readFileSync(path.join(root, 'src', 'm365', entry.file), 'utf8');
    const body = ts.slice(Math.max(0, ts.indexOf('commandAction(')));
    const found = [['handleRejectedODataJsonPromise', 'json'], ['handleRejectedODataPromise', 'odata'], ['handleRejectedPromise', 'plain'], ['handleError(', 'plain']]
      .map(([k, v]) => [body.indexOf(k), v]).filter(([i]) => i >= 0).sort((a, b) => a[0] - b[0]);
    entry.errors = found.length ? found[0][1] : 'none';
    // VivaEngageCommand overrides handleRejectedODataJsonPromise
    if (entry.errors === 'json' && entry.bases.includes('VivaEngageCommand')) entry.errors = 'viva';
  }
  if (cmd.schema) {
    const shape = cmd.schema._zod.def.shape;
    entry.strict = cmd.schema._zod.def.catchall?._zod.def.type === 'never';
    if (cmd.schema._zod.def.catchall && !entry.strict) entry.loose = true;
    entry.fields = Object.fromEntries(Object.entries(shape).map(([k, v]) => [k, field(v)]));
    const fieldChecks = checks(cmd.schema._zod.def);
    if (fieldChecks.length) entry.schemaChecks = fieldChecks.map(r => { const rule = r.k === 'custom' ? classify(r) : undefined; return rule ? { ...r, rule } : r; });
    const refined = cmd.getRefinedSchema(cmd.schema);
    if (refined && refined !== cmd.schema) entry.refines = objectRefines(refined);
  } else {
    entry.legacy = true;
    if (cmd.validators.length > 4) entry.customValidators = cmd.validators.length - 4;
    entry.types = cmd.types;
    entry.optionSets = (cmd.optionSets || []).map(s => ({ options: s.options, runsWhen: src(s.runsWhen) }));
  }
  out.push(entry);
}
out.sort((a, b) => a.name.localeCompare(b.name));
const pkg = JSON.parse(fs.readFileSync(path.join(root, 'package.json'), 'utf8'));
process.stdout.write(JSON.stringify({ version: pkg.version, description: pkg.description, commands: out }, null, 1) + '\n');
