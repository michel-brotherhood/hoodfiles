/* HoodFiles — comportamento da página. Sem dependências, sem requisições externas. */
(() => {
  'use strict';

  const reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)');

  /* ----- Menu móvel ----- */
  const toggle = document.querySelector('[data-nav-toggle]');
  const nav = document.querySelector('[data-nav]');
  if (toggle && nav) {
    const label = toggle.querySelector('.sr-only');
    const setNav = (open) => {
      toggle.setAttribute('aria-expanded', String(open));
      nav.classList.toggle('is-open', open);
      if (label) label.textContent = open ? 'Fechar menu' : 'Abrir menu';
    };
    toggle.addEventListener('click', () => setNav(toggle.getAttribute('aria-expanded') !== 'true'));
    nav.addEventListener('click', (e) => { if (e.target.closest('a')) setNav(false); });
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && toggle.getAttribute('aria-expanded') === 'true') { setNav(false); toggle.focus(); }
    });
    window.matchMedia('(min-width: 861px)').addEventListener('change', (e) => { if (e.matches) setNav(false); });
  }

  /* ----- Link ativo conforme a seção ----- */
  const links = [...document.querySelectorAll('.nav__list a[href^="#"]')];
  if (links.length && 'IntersectionObserver' in window) {
    const byId = new Map(links.map((a) => [a.getAttribute('href').slice(1), a]));
    const io = new IntersectionObserver((entries) => {
      entries.forEach((en) => {
        const a = byId.get(en.target.id);
        if (!a) return;
        if (en.isIntersecting) {
          links.forEach((l) => { l.classList.remove('is-active'); l.removeAttribute('aria-current'); });
          a.classList.add('is-active');
          a.setAttribute('aria-current', 'location');
        }
      });
    }, { rootMargin: '-45% 0px -50% 0px' });
    byId.forEach((_, id) => { const s = document.getElementById(id); if (s) io.observe(s); });
  }

  /* ----- Revelar ao rolar (só se houver suporte; sem JS tudo já aparece) ----- */
  if ('IntersectionObserver' in window && !reduceMotion.matches) {
    const targets = document.querySelectorAll('.section__head, .card, .plat, .timeline li, .download, .demo');
    const rev = new IntersectionObserver((entries, obs) => {
      entries.forEach((en) => { if (en.isIntersecting) { en.target.classList.add('in'); obs.unobserve(en.target); } });
    }, { rootMargin: '0px 0px -8% 0px', threshold: 0.08 });
    targets.forEach((el, i) => { el.classList.add('reveal'); el.style.transitionDelay = `${(i % 3) * 70}ms`; rev.observe(el); });
  }

  /* ----- Demonstração do fluxo (ilustrativa) ----- */
  const demo = document.querySelector('[data-demo]');
  if (demo) {
    const buttons = [...demo.querySelectorAll('[data-go]')];
    const toggleBtn = demo.querySelector('[data-toggle]');
    const live = demo.querySelector('[data-live]');
    const labels = [
      'Passo 1: segure o arquivo e comece a arrastar.',
      'Passo 2: acione o atalho; o menu flutuante aparece perto do cursor.',
      'Passo 3: solte o arquivo na operação escolhida.',
      'Pronto: o arquivo convertido foi salvo ao lado do original.',
    ];
    const durations = [2400, 2900, 2400, 3300];
    let step = 0;
    let timer = null;
    let playing = !reduceMotion.matches;
    let visible = false;

    const render = (n, announce) => {
      step = n;
      demo.dataset.step = String(n);
      buttons.forEach((b, i) => {
        if (i === Math.min(n, 2)) b.setAttribute('aria-current', 'step'); else b.removeAttribute('aria-current');
      });
      if (announce && live) live.textContent = labels[n];
    };
    const schedule = () => {
      clearTimeout(timer);
      if (!playing || !visible) return;
      timer = setTimeout(() => { render((step + 1) % labels.length); schedule(); }, durations[step]);
    };
    const setPlaying = (value) => {
      playing = value;
      if (toggleBtn) toggleBtn.textContent = value ? 'Pausar animação' : 'Reproduzir animação';
      schedule();
    };

    buttons.forEach((b, i) => b.addEventListener('click', () => { setPlaying(false); render(i, true); }));
    if (toggleBtn) toggleBtn.addEventListener('click', () => {
      if (!playing && step === 3) render(0);
      setPlaying(!playing);
    });

    if ('IntersectionObserver' in window) {
      new IntersectionObserver((entries) => {
        visible = entries[0].isIntersecting;
        schedule();
      }, { threshold: 0.35 }).observe(demo);
    } else {
      visible = true;
    }
    render(0);
    setPlaying(playing);
  }
})();
