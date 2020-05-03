<script>
  import { getDaily } from "../notes";
  import Commit from "./Commit.svelte";

  export let config;
</script>

<div>
  {#each config.commits as commit}
    <Commit {commit} />
  {/each}
</div>
