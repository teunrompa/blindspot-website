<script lang="ts">
  let mouse_x = 0;
  let mouse_y = 0;

  function handle_mouse_move(e: MouseEvent) {
    mouse_x = (e.clientX / window.innerWidth - 0.5) * 2;
    mouse_y = (e.clientY / window.innerHeight - 0.5) * 2;
  }

  $: translateX = mouse_x;
  $: translateY = mouse_y;
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="landing" on:mousemove={handle_mouse_move}>
  <div class="specimen-frame">
    <div class="spec-corner tl"></div>
    <div class="spec-corner tr"></div>
    <div class="spec-corner bl"></div>
    <div class="spec-corner br"></div>

    <div class="spec-meta">
      <span>SPEC. №</span>
      <span>BS—001</span>
    </div>

    <div class="title-block">
      <h1
        class="word"
        style="transform: translate({translateX * 12}px, {translateY * 12}px)"
      >
        BLIND
      </h1>
      <div
        class="title-bar"
        style="transform: translate({translateX * 4}px, {translateY * 4}px)"
      ></div>
      <h1
        class="word"
        style="transform: translate({-translateX * 12}px, {-translateY * 12}px)"
      >
        SPOT
      </h1>
    </div>

    <p
      class="caption"
      style="transform: translate({translateX * 6}px, {translateY * 6}px)"
    >
      Highlighting the unknown
    </p>

    <div class="spec-footer">
      <span>OBS. 2026.05.19</span>
      <span>HIVE—MIND</span>
    </div>
  </div>

  <div class="hint">[ click anywhere to enter ]</div>
</div>

<style>
  .landing {
    position: relative;
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 56px;
    background:
      /* faint top-light */
      radial-gradient(
        ellipse at 50% -20%,
        color-mix(in srgb, var(--color-secondary) 8%, transparent) 0%,
        transparent 60%
      ),
      /* faint olive haze low-center */
        radial-gradient(
          ellipse at 50% 110%,
          color-mix(in srgb, var(--color-olive) 14%, transparent) 0%,
          transparent 55%
        ),
      /* fine grain */
        url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='200' height='200'><filter id='n'><feTurbulence baseFrequency='0.9' numOctaves='2' stitchTiles='stitch'/><feColorMatrix values='0 0 0 0 1  0 0 0 0 1  0 0 0 0 1  0 0 0 0.04 0'/></filter><rect width='100%' height='100%' filter='url(%23n)'/></svg>"),
      var(--color-primary);
  }

  /* ── Specimen frame ── */
  .specimen-frame {
    position: relative;
    padding: 72px 96px 64px;
    border: 1px solid var(--color-border);
  }

  .spec-corner {
    position: absolute;
    width: 14px;
    height: 14px;
    border: 1px solid var(--color-highlight);
  }
  .spec-corner.tl {
    top: -1px;
    left: -1px;
    border-right: 0;
    border-bottom: 0;
  }
  .spec-corner.tr {
    top: -1px;
    right: -1px;
    border-left: 0;
    border-bottom: 0;
  }
  .spec-corner.bl {
    bottom: -1px;
    left: -1px;
    border-right: 0;
    border-top: 0;
  }
  .spec-corner.br {
    bottom: -1px;
    right: -1px;
    border-left: 0;
    border-top: 0;
  }

  .spec-meta,
  .spec-footer {
    position: absolute;
    left: 24px;
    right: 24px;
    display: flex;
    justify-content: space-between;
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.3em;
    color: var(--color-muted);
    text-transform: uppercase;
    pointer-events: none;
  }
  .spec-meta {
    top: -6px;
  }
  .spec-footer {
    bottom: -6px;
  }

  /* Inset chips so they break the border line cleanly */
  .spec-meta span,
  .spec-footer span {
    background: var(--color-primary);
    padding: 0 10px;
  }

  /* ── Title ── */
  .title-block {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 10px;
  }

  .word {
    font-family: var(--font-display);
    font-size: clamp(72px, 12vw, 114px);
    line-height: 0.86;
    letter-spacing: 0.05em;
    color: var(--color-secondary);
    font-weight: 400;
    transition: transform 0.15s ease-out;
    will-change: transform;
  }

  .title-bar {
    width: 72px;
    height: 2px;
    background: var(--color-highlight);
    transition: transform 0.15s ease-out;
    will-change: transform;
  }

  /* ── Caption ── */
  .caption {
    margin-top: 28px;
    text-align: center;
    font-size: clamp(0.6rem, 1.5vw, 0.75rem);
    letter-spacing: 0.4em;
    text-transform: uppercase;
    color: var(--color-muted);
    font-family: var(--font-mono);
    transition: transform 0.15s ease-out;
    will-change: transform;
  }

  /* ── Hint (outside frame) ── */
  .hint {
    font-size: 10px;
    letter-spacing: 0.35em;
    color: var(--color-dim);
    text-transform: uppercase;
    font-family: var(--font-mono);
    animation: blink 2.5s step-end infinite;
  }

  @keyframes blink {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.2;
    }
  }

  /* ── Mobile ── */
  @media (max-width: 640px) {
    .specimen-frame {
      padding: 56px 32px 48px;
    }
    .spec-meta,
    .spec-footer {
      left: 12px;
      right: 12px;
      font-size: 8px;
      letter-spacing: 0.22em;
    }
  }
</style>
