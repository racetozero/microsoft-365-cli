// Runs the upstream CLI for Microsoft 365 with its HTTP layer routed to the
// parity mock server, exactly like the Rust binary built with
// `--features parity`.
//
//   M365_PARITY_MOCK=http://127.0.0.1:PORT node upstream-harness.mjs <upstream dir> <args...>
//
// Access tokens are not requested: `auth.ensureAccessToken` returns the token
// stored in the connection file (or a fixed mock token), as the Rust parity
// build does.
import path from 'path';
import { pathToFileURL } from 'url';

const [root, ...args] = process.argv.slice(2);
const dist = path.join(path.resolve(root), 'dist');
const imp = p => import(pathToFileURL(path.join(dist, p)));
const { default: request } = await imp('request.js');
const { default: auth } = await imp('Auth.js');
const { cli } = await imp('cli/cli.js');
const { msalCachePlugin } = await imp('auth/msalCachePlugin.js');
const mock = process.env.M365_PARITY_MOCK;

// Never open a browser (`cli issue`, `docs`, device-code login): the Rust
// parity build does the same.
const { browserUtil } = await imp('utils/browserUtil.js');
browserUtil.open = async () => { };

// Never touch the OS keychain: use upstream's plain-file fallback in the
// sandbox HOME, which is what the Rust port uses.
msalCachePlugin.createNativePersistence = async () => msalCachePlugin.createFileFallback();

// Mirrors the Rust parity build: an existing token is returned; otherwise the
// fixed M365_PARITY_TOKEN is "acquired" and the connection updated the way
// the real ensureAccessToken does.
const { accessToken: accessTokenUtil } = await imp('utils/accessToken.js');
auth.ensureAccessToken = async (resource) => {
  const existing = auth.connection.accessTokens?.[resource]?.accessToken;
  if (existing) return existing;
  const token = process.env.M365_PARITY_TOKEN || 'parity-mock-token';
  auth.connection.accessTokens[resource] = { expiresOn: '2099-01-01T00:00:00.000Z', accessToken: token };
  auth.connection.active = true;
  auth.connection.identityName = accessTokenUtil.getUserNameFromAccessToken(token);
  auth.connection.identityId = accessTokenUtil.getUserIdFromAccessToken(token) || auth.connection.appId;
  auth.connection.identityTenantId = accessTokenUtil.getTenantIdFromAccessToken(token);
  auth.connection.name = auth.connection.name || auth.connection.identityId;
  try { await auth.storeConnectionInfo(); } catch { /* as upstream */ }
  return token;
};

function serializeBody(data) {
  if (data === undefined || data === null) return null;
  if (typeof data === 'string') return data;
  if (Buffer.isBuffer(data)) return `<${data.length} bytes>`;
  if (typeof data.pipe === 'function') return '<stream>';
  return JSON.stringify(data);
}

// axios interceptors (debug logging) run around the mock transport
const reqInterceptors = [], resInterceptors = [];
const send = async (config) => {
  for (const f of reqInterceptors) config = await f(config);
  try {
    let res = await transport(config);
    for (const [ok] of resInterceptors) if (ok) res = await ok(res);
    return res;
  }
  catch (err) {
    let e = err;
    for (const [, bad] of resInterceptors) {
      if (!bad) continue;
      try { await bad(e); } catch (x) { e = x; }
    }
    throw e;
  }
};
send.interceptors = {
  request: { use: f => { reqInterceptors.push(f); } },
  response: { use: (ok, bad) => { resInterceptors.push([ok, bad]); } }
};
request.req = send;
async function transport(config) {
  const headers = {};
  for (const [k, v] of Object.entries(config.headers || {})) {
    if (v === undefined) continue;
    headers[k.toLowerCase()] = String(v);
  }
  const desc = {
    method: (config.method || 'get').toUpperCase(),
    url: config.url,
    headers,
    body: serializeBody(config.data)
  };
  const res = await fetch(`${mock}/__m365`, { method: 'POST', body: JSON.stringify(desc) });
  const r = await res.json();
  const text = r.body ?? '';
  let data = text;
  if (config.responseType === 'json' && text) {
    try { data = JSON.parse(text); } catch { data = text; }
  }
  if (config.responseType === 'stream') {
    const { Readable } = await import('stream');
    data = Readable.from([Buffer.from(text)]);
  }
  const response = { data, status: r.status, statusText: r.statusText || '', headers: r.headers || {}, config };
  if (r.status >= 200 && r.status < 300) {
    return config.responseType === 'stream' || config.fullResponse ? response : response;
  }
  const err = new Error(`Request failed with status code ${r.status}`);
  err.response = response;
  err.config = config;
  if (data && config.responseType !== 'stream') {
    err.error = JSON.parse(JSON.stringify(data));
  }
  throw err;
}

// as upstream's index.js: keep piped output complete when process.exit runs
if (process.stdout._handle) process.stdout._handle.setBlocking(true);
if (!process.env.CLIMICROSOFT365_NOUPDATE) process.env.CLIMICROSOFT365_NOUPDATE = '1';
try {
  await cli.execute(args);
}
catch (err) {
  if (err instanceof Error && err.name === 'ExitPromptError') {
    process.exit(1);
  }
  await cli.closeWithError(err, cli.optionsFromArgs || { options: {} });
}
