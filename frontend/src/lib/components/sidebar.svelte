<script lang="ts">
  import { Pages } from "../types/data";
  import Events from "../../pages/events.svelte";
  import About from "../../pages/about.svelte";
  import { onMount } from "svelte";

  export let currentPage: Pages;

  onMount(() => {
    goToPage(Pages.Events);
  });

  function goToPage(page: Pages) {
    currentPage = page;
  }
</script>

<div class="sidebar-layout">
  <aside class="sidebar">
    <div class="sb-header">
      <div class="sb-logo">BLINDSPOT</div>
      <div class="sb-spec">SPEC. № BS—001</div>
    </div>

    <div class="sb-bar"></div>

    <nav>
      <button
        class="nav-btn"
        class:active={currentPage === Pages.Events}
        onclick={() => goToPage(Pages.Events)}
      >
        <span class="nav-index">01</span>
        <span class="nav-label">Events</span>
      </button>
      <button
        class="nav-btn"
        class:active={currentPage === Pages.About}
        onclick={() => goToPage(Pages.About)}
      >
        <span class="nav-index">02</span>
        <span class="nav-label">About</span>
      </button>
    </nav>

    <div class="sb-footer">
      <div class="sb-footer-label">OBS.</div>
      <div>BLINDSPOT</div>
      <div>COLLECTIVE</div>
      <div class="sb-footer-meta">— HIVE-MIND —</div>
    </div>
  </aside>

  <main class="page-container">
    {#if currentPage === Pages.Events}
      <Events />
    {:else if currentPage === Pages.About}
      <About />
    {/if}
  </main>
</div>

<style>
  .sidebar-layout {
    display: flex;
    width: 100%;
    height: 100%;
  }

  .sidebar {
    width: 200px;
    min-width: 200px;
    background: var(--color-primary);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    padding: 28px 20px;
  }

  /* ── Header ── */
  .sb-header {
    margin-bottom: 8px;
  }

  .sb-logo {
    font-family: var(--font-display);
    font-size: 18px;
    letter-spacing: 0.14em;
    color: var(--color-secondary);
    line-height: 1;
  }

  .sb-spec {
    margin-top: 6px;
    font-family: var(--font-mono);
    font-size: 8px;
    letter-spacing: 0.3em;
    color: var(--color-dim);
    text-transform: uppercase;
  }

  .sb-bar {
    width: 28px;
    height: 1px;
    background: var(--color-highlight);
    margin: 18px 0 36px;
  }

  /* ── Nav ── */
  nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-btn {
    display: grid;
    grid-template-columns: 24px 1fr;
    align-items: center;
    gap: 10px;
    background: none;
    border: none;
    border-left: 1px solid transparent;
    cursor: none;
    text-align: left;
    padding: 9px 10px;
    font-family: var(--font-mono);
    font-size: 10px;
    letter-spacing: 0.28em;
    text-transform: uppercase;
    color: var(--color-muted);
    transition:
      color 0.2s,
      border-color 0.2s,
      background 0.2s;
  }

  .nav-index {
    font-size: 8px;
    letter-spacing: 0.1em;
    color: var(--color-dim);
    transition: color 0.2s;
  }

  .nav-btn:hover {
    color: var(--color-secondary);
    border-left-color: var(--color-border);
  }

  .nav-btn:hover .nav-index {
    color: var(--color-muted);
  }

  .nav-btn.active {
    color: var(--color-secondary);
    border-left-color: var(--color-highlight);
    background: color-mix(in srgb, var(--color-highlight) 6%, transparent);
  }

  .nav-btn.active .nav-index {
    color: var(--color-highlight);
  }

  /* ── Footer ── */
  .sb-footer {
    margin-top: auto;
    font-family: var(--font-mono);
    font-size: 8px;
    letter-spacing: 0.22em;
    text-transform: uppercase;
    color: var(--color-muted);
    line-height: 1.9;
  }

  .sb-footer-label {
    color: var(--color-dim);
    margin-bottom: 6px;
  }

  .sb-footer-meta {
    margin-top: 10px;
    color: var(--color-dim);
    letter-spacing: 0.3em;
  }

  /* ── Page container ── */
  .page-container {
    flex: 1;
    overflow-y: auto;
    padding: 28px 36px;
    background: var(--color-primary);
  }
</style>
