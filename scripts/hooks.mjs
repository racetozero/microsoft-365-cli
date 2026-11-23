// Lists the validation hooks a command needs: field checks that are not
// well-known validators (`field` hook, by index) and object refinements that
// are not structural rules (`refine` hook, by index).
//
//   node scripts/hooks.mjs "entra app"
import fs from 'fs';
const prefix = process.argv[2] || '';
const m = JSON.parse(fs.readFileSync(process.env.M365_MANIFEST || new URL('../target/commands.json', import.meta.url), 'utf8')) // node scripts/extract-upstream.mjs /tmp/cli-microsoft365 > target/commands.json;
const known = /^\(?(\w+)\)? => (?:\1 === undefined \|\| |!\1 \|\| )?validation\.(\w+)\(\1\)(?: === true)?$/;
for (const c of m.commands.filter(c => c.name === prefix || c.name.startsWith(prefix + ' '))) {
  const lines = [];
  let i = 0;
  for (const [f, spec] of Object.entries(c.fields || {})) {
    for (const k of spec.checks || []) {
      if (k.k !== 'custom' || known.test(k.src || '')) continue;
      if (k.expr) { i++; continue; } // compiled, see scripts/js-expr.mjs
      lines.push(`  field ${i++} ${f}${k.async ? ' (async)' : ''}: ${k.src}`);
    }
  }
  (c.refines || c.schemaChecks || []).forEach((r, j) => {
    if (!r.rule && !r.expr && (r.k === undefined || r.k === 'custom')) lines.push(`  refine ${j}: ${r.src}${r.msg === '<dynamic>' ? '  [dynamic message]' : ''}`);
  });
  if (c.legacy && c.customValidators) lines.push(`  validate: ${c.customValidators} custom validator(s)`);
  if (lines.length) console.log(`${c.name}\n${lines.join('\n')}`);
}
