<script>
  import { getDaily } from "@gtm/notes";
  import Commit from "./Commit.svelte";

  export let commits;
</script>

<div>
  {#each commits as commit}
    <Commit {commit} />
  {/each}
</div>
