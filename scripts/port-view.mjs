// Prints upstream command sources without boilerplate, to read while porting.
//
//   node scripts/port-view.mjs "entra user" [/tmp/cli-microsoft365]
//
// Drops imports, the name/description/schema getters, telemetry and option
// declarations (those come from the extracted commands.json) and blank lines.
import fs from 'fs';
import path from 'path';

const prefix = process.argv[2] || '';
const root = process.argv[3] || '/tmp/cli-microsoft365';
const manifest = JSON.parse(fs.readFileSync(process.env.M365_MANIFEST || new URL('../target/commands.json', import.meta.url), 'utf8')) // node scripts/extract-upstream.mjs /tmp/cli-microsoft365 > target/commands.json;
const cmds = manifest.commands.filter(c => c.name === prefix || c.name.startsWith(prefix + ' '));
for (const c of cmds) {
  const file = path.join(root, 'src', 'm365', c.file);
  let src = fs.readFileSync(file, 'utf8');
  src = src.replace(/^import[\s\S]*?;\s*$/gm, '');
  src = src.replace(/  public get (name|description|schema)\(\)[^{]*\{[\s\S]*?\n  \}\n/g, '');
  src = src.replace(/  public alias\(\)[^{]*\{[\s\S]*?\n  \}\n/g, '');
  src = src.replace(/  public defaultProperties\(\)[^{]*\{[\s\S]*?\n  \}\n/g, '');
  src = src.replace(/  #initTelemetry\(\)[^{]*\{[\s\S]*?\n  \}\n/g, '');
  src = src.replace(/  #initOptions\(\)[^{]*\{[\s\S]*?\n  \}\n/g, '');
  src = src.replace(/  #initTypes\(\)[^{]*\{[\s\S]*?\n  \}\n/g, '');
  src = src.replace(/  #initOptionSets\(\)[^{]*\{[\s\S]*?\n  \}\n/g, '');
  src = src.replace(/  constructor\(\) \{\n    super\(\);\n([\s\S]*?)  \}\n/g, '');
  src = src.replace(/^export declare type Options[^\n]*\n|^declare type Options[^\n]*\n|^interface CommandArgs \{\n  options: Options;\n\}\n/gm, '');
  src = src.replace(/^export default new \w+\(\);\s*$/gm, '');
  src = src.replace(/\n\s*\n/g, '\n');
  const extra = [];
  if (c.defaultProperties) extra.push(`defaultProperties: ${c.defaultProperties.join(',')}`);
  if (c.aliases.length) extra.push(`aliases: ${c.aliases.join(', ')}`);
  extra.push(`bases: ${c.bases.join('>')}`);
  console.log(`\n===== ${c.name} (${c.file}) [${extra.join(' | ')}]`);
  console.log(src.trim());
}
