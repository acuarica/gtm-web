<script>
  import Progress from "./Progress.svelte";

  export let promise;
</script>

{#await promise}
  <Progress />
{:then value}
  <slot {value} />
{:catch error}
  <p style="color: red">Got {JSON.stringify(error)} while performing fetch</p>
{/await}
