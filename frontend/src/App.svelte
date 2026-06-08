<script lang="ts">
  import { onMount } from "svelte";
  import Main from "./pages/main.svelte";
  import Landing from "./pages/landing.svelte";
  import { Pages } from "./lib/types/data";

  let currentPage = $state(Pages.Landing);

  const LERP = 0.12;
  let cursor_x = 0;
  let cursor_y = 0;
  let dot_x = $state(0);
  let dot_y = $state(0);

  onMount(() => {
    window.addEventListener("mousemove", (e) => {
      cursor_x = e.clientX;
      cursor_y = e.clientY;
    });

    let raf: number;
    function loop() {
      dot_x += (cursor_x - dot_x) * LERP;
      dot_y += (cursor_y - dot_y) * LERP;
      raf = requestAnimationFrame(loop);
    }
    loop();

    return () => cancelAnimationFrame(raf);
  });

  let cursor_scale = $state(1);
  const pulse_time = 0.5;

  function pulseClick() {
    if (currentPage === Pages.Landing) {
      currentPage = Pages.Home;
    }
    const t = pulse_time * 1000;
    cursor_scale = 0.5;
    setTimeout(() => {
      cursor_scale = 1.2;
    }, t * 0.2);
    setTimeout(() => {
      cursor_scale = 1;
    }, t * 0.8);
  }
</script>

<div
  class="cursor-dot"
  style="left: {dot_x}px; top: {dot_y}px; transform: translate(-50%, -50%) scale({cursor_scale})"
></div>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="app" onmousedown={pulseClick}>
  {#if currentPage === Pages.Landing}
    <Landing />
  {:else if currentPage === Pages.Home}
    <Main {currentPage} />
  {/if}
</div>

<style>
  :global(*, *::before, *::after) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    cursor: none;
    color: var(--color-primary);
    font-family: "Courier New", Courier, monospace;
    overflow: hidden;
  }

  .app {
    width: 100vw;
    height: 100vh;
    position: relative;
    overflow: hidden;
  }
</style>
