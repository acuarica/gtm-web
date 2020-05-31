<script>
  export let icon;
  let path = [];
  let classes = "";
  let viewBox = "";
  $: viewBox = "0 0 " + icon.icon[0] + " " + icon.icon[1];
  $: classes = "fa-svelte " + ($$props.class ? $$props.class : "");
  $: path = icon.icon[4];
</script>

<style>
  .fa-svelte {
    overflow: visible;
    display: inline-block;
  }
</style>

<svg
  aria-hidden="true"
  class={classes}
  role="img"
  xmlns="http://www.w3.org/2000/svg"
  {viewBox}>
  <path fill="currentColor" d={path} />
</svg>
