<script>
  import { getDaily } from "../gtm";
  import Commit from "./Commit.svelte";

  export let commits = null;
  export const map = null;
</script>

<div>
  {#each commits as commit}
    <Commit {commit} />
  {/each}
</div>
