<script>
  import Box from "./Box.svelte";
  import Commit from "./Commit.svelte";

  export let commits;
</script>

<Box class="p-3 divide-y-1" style="width: 300px">
  {#each commits as commit}
    <Commit {commit} />
  {/each}
</Box>
