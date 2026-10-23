const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const rows = document.getElementById('log');
let last = null;

function addRow(e) {
  const tr = document.createElement('tr');
  for (const [txt, cls] of [[String(e.t_ms), ''], [e.kind, ''], [JSON.stringify(e.detail, null, 1), 'd']]) {
    const td = document.createElement('td');
    td.textContent = txt;
    if (cls) td.className = cls;
    tr.appendChild(td);
  }
  rows.prepend(tr);
}

async function refresh() {
  last = await invoke('get_report');
  const c = last.config, env = last.env;
  document.getElementById('info').textContent =
    `${env.os}/${env.arch} · Tauri ${last.tauri} · atalho ${c.shortcut} (${c.mode}) · Escape=${c.escape} · ` +
    `focusable=${c.focusable} · elevado=${env.elevated ?? 'n/a'} · log: ${last.log_file ?? 'n/d'}` +
    (env.details['os_release.PRETTY_NAME'] ? ` · ${env.details['os_release.PRETTY_NAME']} · sessão=${env.details.XDG_SESSION_TYPE} · desktop=${env.details.XDG_CURRENT_DESKTOP}` : '');
  rows.replaceChildren();
  last.log.forEach(addRow);
}

document.getElementById('refresh').onclick = refresh;
document.getElementById('copy').onclick = async () => {
  await refresh();
  const text = JSON.stringify(last, null, 2);
  const s = document.getElementById('status');
  try { await navigator.clipboard.writeText(text); s.textContent = 'copiado'; }
  catch { s.textContent = 'clipboard indisponível; use o arquivo de log'; }
};
listen('poc:event', (ev) => addRow(ev.payload));
refresh();
