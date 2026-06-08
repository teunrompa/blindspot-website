<script lang="ts">
  import Sidebar from "../lib/components/sidebar.svelte";
  import { Pages } from "../lib/types/data";
  import "../app.css";
  export let currentPage: Pages;
</script>

<div class="main">
  <Sidebar bind:currentPage />
</div>

<style>
  .main {
    width: 100vw;
    height: 100vh;
    display: flex;
    overflow: hidden;
  }
</style>
