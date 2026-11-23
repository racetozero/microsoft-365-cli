// Compiles the JavaScript source of a zod refinement predicate into a small
// JSON expression tree that `crates/m365-core/src/expr.rs` evaluates, so most
// command-specific refinements need no hand-written Rust hook.
//
// Only a closed subset of JavaScript is accepted; anything else (closures over
// module constants, `new Date`, `JSON.parse`, regular expressions, `fs`)
// throws and the refinement stays a hook.
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
let ts;
export function init(root) {
  ts = require(`${root}/node_modules/typescript`);
}

const METHODS = new Set(['getTime', 'test', 'filter', 'some', 'every', 'includes', 'indexOf', 'startsWith', 'endsWith', 'toLowerCase', 'toLocaleLowerCase', 'toUpperCase', 'trim', 'split', 'map', 'join']);
// mirrors `validator` in crates/m365-core/src/expr.rs
const VALIDATORS = new Set(['isValidGuid', 'isValidGuidArray', 'isValidTeamsChannelId', 'isValidTeamsChatId', 'isValidUserPrincipalName', 'isValidUserPrincipalNameArray', 'isValidPositiveInteger', 'isValidPositiveIntegerArray', 'isValidISODate', 'isValidISODateDashOnly', 'isValidISODateTime', 'isValidGraphDateTime', 'isValidBoolean', 'isValidSharePointUrl', 'isValidMailNickname', 'isValidISODuration', 'isValidPermission', 'isValidPowerPagesUrl', 'isValidJsonOrEmpty', 'isValidJson', 'isValidTheme', 'isDateInRange']);
const GLOBALS = new Set(['path.resolve', 'path.dirname', 'formatting.splitAndTrim', 'JSON.parse', 'fs.existsSync', 'Number', 'isNaN', 'parseInt', 'Number.isInteger', 'Object.values', 'Object.keys', 'Boolean', 'Array.isArray']);

class Unsupported extends Error {}
const no = (what) => { throw new Unsupported(what); };

function binop(kind) {
  const K = ts.SyntaxKind;
  return {
    [K.BarBarToken]: '||', [K.AmpersandAmpersandToken]: '&&', [K.EqualsEqualsEqualsToken]: '===', [K.ExclamationEqualsEqualsToken]: '!==',
    [K.EqualsEqualsToken]: '==', [K.ExclamationEqualsToken]: '!=', [K.LessThanToken]: '<', [K.LessThanEqualsToken]: '<=',
    [K.GreaterThanToken]: '>', [K.GreaterThanEqualsToken]: '>=', [K.PlusToken]: '+', [K.MinusToken]: '-', [K.QuestionQuestionToken]: '??'
  }[kind];
}

function dotted(n) {
  if (ts.isIdentifier(n)) return n.text;
  if (ts.isPropertyAccessExpression(n)) { const l = dotted(n.expression); return l && `${l}.${n.name.text}`; }
  return undefined;
}

// scope: name -> expr (consts) or { var: name } (lambda params)
function expr(n, scope) {
  const K = ts.SyntaxKind;
  if (ts.isParenthesizedExpression(n)) return expr(n.expression, scope);
  if (ts.isAsExpression?.(n) || ts.isNonNullExpression(n)) return expr(n.expression, scope);
  if (ts.isStringLiteral(n) || ts.isNoSubstitutionTemplateLiteral(n)) return { t: 'lit', v: n.text };
  if (ts.isNumericLiteral(n)) return { t: 'lit', v: Number(n.text) };
  if (n.kind === K.TrueKeyword) return { t: 'lit', v: true };
  if (n.kind === K.FalseKeyword) return { t: 'lit', v: false };
  if (n.kind === K.NullKeyword) return { t: 'null' };
  if (n.kind === K.RegularExpressionLiteral) {
    const m = n.text.match(/^\/(.*)\/([a-z]*)$/s);
    if (!m || /y/.test(m[2])) no('regex flags'); // a literal is fresh per call, so g does not matter
    return { t: 're', p: m[1], f: m[2] };
  }
  if (ts.isIdentifier(n)) {
    if (n.text === 'undefined') return { t: 'undef' };
    if (n.text === 'Boolean' && !Object.hasOwn(scope, 'Boolean')) return { t: 'global' };
    if (Object.hasOwn(scope, n.text)) return scope[n.text];
    return no(`identifier ${n.text}`);
  }
  if (ts.isPrefixUnaryExpression(n)) {
    if (n.operator === K.ExclamationToken) return { t: 'not', e: expr(n.operand, scope) };
    if (n.operator === K.MinusToken) return { t: 'neg', e: expr(n.operand, scope) };
    return no('prefix');
  }
  if (ts.isTypeOfExpression(n)) return { t: 'typeof', e: expr(n.expression, scope) };
  if (ts.isBinaryExpression(n)) {
    const op = binop(n.operatorToken.kind);
    if (!op) return no('binop');
    return { t: 'bin', op, l: expr(n.left, scope), r: expr(n.right, scope) };
  }
  if (ts.isConditionalExpression(n)) return { t: 'if', c: expr(n.condition, scope), a: expr(n.whenTrue, scope), b: expr(n.whenFalse, scope) };
  if (ts.isArrayLiteralExpression(n)) return { t: 'arr', a: n.elements.map(e => expr(e, scope)) };
  if (ts.isPropertyAccessExpression(n) || ts.isPropertyAccessChain?.(n)) {
    return { t: 'get', o: expr(n.expression, scope), k: n.name.text, opt: !!n.questionDotToken };
  }
  if (ts.isElementAccessExpression(n)) {
    return { t: 'idx', o: expr(n.expression, scope), i: expr(n.argumentExpression, scope) };
  }
  if (ts.isArrowFunction(n) || ts.isFunctionExpression(n)) return lambda(n, scope);
  if (ts.isNewExpression(n) && ts.isIdentifier(n.expression) && n.expression.text === 'Date' && (n.arguments || []).length <= 1) {
    return { t: 'date', a: (n.arguments || []).map(a => expr(a, scope)) };
  }
  if (ts.isCallExpression(n)) {
    const callee = n.expression;
    const name = dotted(callee);
    const args = n.arguments.map(a => expr(a, scope));
    if (name && name.startsWith('validation.')) {
      if (!VALIDATORS.has(name.slice(11))) no(name);
      return { t: 'valid', f: name.slice(11), a: args };
    }
    if (name && GLOBALS.has(name)) return { t: 'call', f: name, a: args };
    if (ts.isPropertyAccessExpression(callee) && METHODS.has(callee.name.text)) {
      return { t: 'method', m: callee.name.text, o: expr(callee.expression, scope), a: args };
    }
    return no(`call ${name || '?'}`);
  }
  return no(ts.SyntaxKind[n.kind]);
}

function lambda(n, scope) {
  const params = n.parameters.map(p => {
    if (!ts.isIdentifier(p.name)) no('destructuring');
    return p.name.text;
  });
  const inner = { ...scope };
  for (const p of params) inner[p] = { t: 'var', n: p };
  return { t: 'fn', p: params, b: body(n.body, inner) };
}

function body(b, scope) {
  if (!ts.isBlock(b)) return expr(b, scope);
  return stmts([...b.statements], scope);
}

// `if (c) { return a; } return b;` and `const x = e;` sequences
function stmts(list, scope) {
  if (!list.length) return { t: 'undef' };
  const [s, ...rest] = list;
  if (ts.isReturnStatement(s)) return s.expression ? expr(s.expression, scope) : { t: 'undef' };
  if (ts.isVariableStatement(s)) {
    if (!(s.declarationList.flags & ts.NodeFlags.Const)) no('let');
    // evaluated in order (JSON.parse may throw), bound as variables
    const decls = [...s.declarationList.declarations];
    const bind = (i, sc) => {
      if (i === decls.length) return stmts(rest, sc);
      const d = decls[i];
      if (!ts.isIdentifier(d.name) || !d.initializer) no('decl');
      const v = expr(d.initializer, sc);
      return { t: 'let', n: d.name.text, v, b: bind(i + 1, { ...sc, [d.name.text]: { t: 'var', n: d.name.text } }) };
    };
    return bind(0, scope);
  }
  if (ts.isTryStatement(s)) {
    if (s.finallyBlock || !s.catchClause || s.catchClause.variableDeclaration) no('try form');
    if (!endsWithReturn(s.tryBlock) || !endsWithReturn(s.catchClause.block)) no('try without return');
    return { t: 'try', b: stmts([...s.tryBlock.statements], scope), c: stmts([...s.catchClause.block.statements], scope) };
  }
  if (ts.isExpressionStatement(s)) return { t: 'seq', e: expr(s.expression, scope), b: stmts(rest, scope) };
  if (ts.isIfStatement(s)) {
    const a = stmts(ts.isBlock(s.thenStatement) ? [...s.thenStatement.statements] : [s.thenStatement], scope);
    const elseList = s.elseStatement ? (ts.isBlock(s.elseStatement) ? [...s.elseStatement.statements] : [s.elseStatement]) : [];
    const thenReturns = endsWithReturn(s.thenStatement);
    if (!thenReturns) no('if without return');
    const b = s.elseStatement ? stmts([...elseList, ...rest], scope) : stmts(rest, scope);
    return { t: 'if', c: expr(s.expression, scope), a, b };
  }
  return no(ts.SyntaxKind[s.kind]);
}

function endsWithReturn(s) {
  if (ts.isReturnStatement(s)) return true;
  if (ts.isBlock(s)) return s.statements.length > 0 && endsWithReturn(s.statements[s.statements.length - 1]);
  if (ts.isIfStatement(s)) return !!s.elseStatement && endsWithReturn(s.thenStatement) && endsWithReturn(s.elseStatement);
  if (ts.isTryStatement(s)) return !!s.catchClause && endsWithReturn(s.tryBlock) && endsWithReturn(s.catchClause.block);
  return false;
}

/// The expression tree of a one-parameter predicate, or undefined.
export function compile(source) {
  if (!source || !ts) return undefined;
  try {
    const sf = ts.createSourceFile('x.js', `(${source})`, ts.ScriptTarget.Latest, false, ts.ScriptKind.JS);
    const st = sf.statements[0];
    if (!st || !ts.isExpressionStatement(st)) return undefined;
    let fn = st.expression;
    while (ts.isParenthesizedExpression(fn)) fn = fn.expression;
    if (!(ts.isArrowFunction(fn) || ts.isFunctionExpression(fn)) || fn.parameters.length !== 1) return undefined;
    const p = fn.parameters[0];
    if (!ts.isIdentifier(p.name)) return undefined;
    return body(fn.body, { [p.name.text]: { t: 'arg' } });
  }
  catch (e) {
    if (e instanceof Unsupported) return undefined;
    throw e;
  }
}
