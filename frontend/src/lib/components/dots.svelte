<script lang="ts">
  import { dotPathsJson } from "../../data/dots";

  let dots = dotPathsJson;
</script>

<svg
  class="circle"
  version="1.1"
  width="300"
  height="300"
  viewBox="400 380 300 330"
  xmlns="http://www.w3.org/2000/svg"
>
  {#each dots as dot}
    <circle cx={dot.cx} cy={dot.cy} r={dot.r}></circle>
  {/each}
</svg>

<style>
  .circle {
    fill: #49ea46;
  }
</style>
