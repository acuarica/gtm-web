<script>
  import { getDaily } from "../gtm";
  import Commit from "./Commit.svelte";

  export let commits = null;
  export let map = null;
</script>

<div class="list-group list-group-flush">
  <div id="commitsPlaceholder" />
  {#each commits as commit}
    <Commit {commit} />
  {/each}
</div>
