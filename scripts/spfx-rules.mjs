// Extracts the SharePoint Framework upgrade/doctor rule sets from a built
// upstream checkout as data, and reports how many distinct behaviours
// (`visit` implementations) they contain.
//
//   node scripts/spfx-rules.mjs /tmp/cli-microsoft365 > crates/m365-spfx/src/spfx-rules.json
//   node scripts/spfx-rules.mjs /tmp/cli-microsoft365 --stats
//
// Output: { upgrade: { "<version>": [rule index...] }, doctor: { ... },
// npmDedupe: index, classes: { "<Class>": { base, visit, custom } },
// rules: [{ class, fields, props }] } where `props` holds the evaluated
// getters (id, title, description, resolution, resolutionType, severity,
// file, supersedes) and `visit`/`custom` name the class defining them.
import crypto from 'crypto';
import os from 'os';
import fs from 'fs';
import path from 'path';
import { pathToFileURL } from 'url';

const [root, ...flags] = process.argv.slice(2);
const dist = path.join(path.resolve(root), 'dist/m365/spfx/commands/project');
const imp = p => import(pathToFileURL(path.join(dist, p)).href);
const hash = s => crypto.createHash('sha1').update(s).digest('hex').slice(0, 10);
// getters that use `os.EOL` must follow the platform the port runs on: the
// extracted text carries a marker that the Rust engine replaces
Object.defineProperty(os, 'EOL', { value: '\u0000EOL\u0000' });
const GETTERS = ['id', 'title', 'description', 'resolution', 'resolutionType', 'severity', 'file', 'supersedes'];
const classes = {};

// classes are keyed by name; a second, different class with the same name
// (project-doctor re-declares DependencyRule and some rules) gets `@doctor`
const names = new Map();
const key = ctor => {
  if (!names.has(ctor)) {
    const taken = [...names.values()].includes(ctor.name);
    names.set(ctor, taken ? `${ctor.name}@doctor` : ctor.name);
  }
  return names.get(ctor);
};

function chain(o) {
  const out = [];
  for (let p = Object.getPrototypeOf(o); p && p.constructor !== Object; p = Object.getPrototypeOf(p)) out.push(key(p.constructor));
  return out;
}

// where a member is defined along the prototype chain
function owner(o, name) {
  for (let p = Object.getPrototypeOf(o); p && p.constructor !== Object; p = Object.getPrototypeOf(p)) {
    const d = Object.getOwnPropertyDescriptor(p, name);
    if (d) return { cls: key(p.constructor), src: String(d.get || d.value) };
  }
  return null;
}

function describe(r) {
  const cls = key(r.constructor);
  if (!classes[cls]) {
    const visit = owner(r, 'visit');
    const custom = owner(r, 'customCondition');
    const getters = Object.fromEntries(GETTERS.map(g => [g, owner(r, g)]).filter(([, v]) => v).map(([g, v]) => [g, { cls: v.cls, hash: hash(v.src), reads: /this\.(?!packageName|packageVersion|isDevDep|isOptional|add|isOverride|version|filePath|contents|_)\w+/.test(v.src) ? v.src.match(/this\.\w+/g) : undefined }]));
    classes[cls] = { chain: chain(r), visit: visit && { cls: visit.cls, hash: hash(visit.src) }, custom: custom && { cls: custom.cls, hash: hash(custom.src) }, getters };
  }
  const props = {};
  for (const g of GETTERS) {
    try { props[g] = r[g]; } catch (e) { props[g] = { error: e.message }; }
  }
  return { class: cls, fields: JSON.parse(JSON.stringify({ ...r })), props };
}

const upgrade = {}, doctor = {};
for (const f of fs.readdirSync(path.join(dist, 'project-upgrade')).filter(f => /^upgrade-.*\.js$/.test(f))) {
  upgrade[f.slice(8, -3)] = (await imp(`project-upgrade/${f}`)).default.map(describe);
}
for (const f of fs.readdirSync(path.join(dist, 'project-doctor')).filter(f => /^doctor-.*\.js$/.test(f))) {
  doctor[f.slice(7, -3)] = (await imp(`project-doctor/${f}`)).default.map(describe);
}
doctor.generic = (await imp('project-doctor/generic-rules.js')).rules.map(describe);
const { FN017001_MISC_npm_dedupe } = await imp('project-upgrade/rules/FN017001_MISC_npm_dedupe.js');
const npmDedupe = describe(new FN017001_MISC_npm_dedupe());

// the SharePoint Framework compatibility matrix (spfx doctor, workflows)
const matrix = await import(pathToFileURL(path.join(path.resolve(root), 'dist/m365/spfx/commands/SpfxCompatibilityMatrix.js')).href);
fs.writeFileSync(path.join(path.dirname(new URL(import.meta.url).pathname), '../crates/m365-spfx/src/spfx-matrix.json'), JSON.stringify({ versions: matrix.versions, SharePointVersion: matrix.SharePointVersion }));

if (flags.includes('--stats')) {
  const byVisit = {};
  for (const [cls, c] of Object.entries(classes)) {
    const k = c.visit ? `${c.visit.cls}:${c.visit.hash}` : 'none';
    (byVisit[k] ||= []).push(cls);
  }
  const shared = Object.entries(byVisit).filter(([, v]) => v.length > 1);
  const own = Object.entries(byVisit).filter(([, v]) => v.length === 1);
  console.log(`${Object.keys(classes).length} rule classes, ${Object.keys(byVisit).length} distinct visit implementations`);
  for (const [k, v] of shared.sort((a, b) => b[1].length - a[1].length)) console.log(`  ${k} x${v.length}`);
  const ownByBase = {};
  for (const [, [cls]] of own) { const b = classes[cls].chain[1] || '?'; (ownByBase[b] ||= []).push(cls); }
  for (const [b, v] of Object.entries(ownByBase)) console.log(`  own visit, base ${b}: ${v.length}`);
  const custom = Object.entries(classes).filter(([, c]) => c.custom && c.custom.cls !== 'DependencyRule');
  console.log(`  customCondition overrides: ${custom.map(([k]) => k).join(', ')}`);
  const stateful = Object.entries(classes).flatMap(([k, c]) => Object.entries(c.getters).filter(([, g]) => g.reads).map(([g, v]) => `${k}.${g} reads ${v.reads.join(',')}`));
  console.log(`  getters reading other state: ${stateful.length}`);
  for (const s of stateful) console.log(`    ${s}`);
  process.exit(0);
}
// deduplicated: unique rule instances, versions as index lists, and per
// class only what the Rust engine dispatches on (who defines `visit` and
// `customCondition`)
const rules = [], index = new Map();
const ref = r => { const k = JSON.stringify(r); if (!index.has(k)) { index.set(k, rules.length); rules.push(r); } return index.get(k); };
const table = sets => Object.fromEntries(Object.entries(sets).map(([v, rs]) => [v, rs.map(ref)]));
const out = { upgrade: table(upgrade), doctor: table(doctor), npmDedupe: ref(npmDedupe), classes: Object.fromEntries(Object.entries(classes).map(([k, c]) => [k, { base: c.chain[1], visit: c.visit?.cls, custom: c.custom?.cls }])), rules };
console.log(JSON.stringify(out));
