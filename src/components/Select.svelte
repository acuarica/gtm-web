<script>
  import { onMount, afterUpdate } from "svelte";
  // import JQ from "jquery";
  // import * as JQ from "jquery";
  // import "bootstrap";
  // import "bootstrap-select";

  export let options;
  export let classes = "";
  export let multiple = false;

  let select;

  // $: if (options.length > 0) {
  //   JQ(select).selectpicker();
  // }

  onMount(() => {
    // JQ(select).selectpicker();
  });

  // afterUpdate(() => {
  // JQ(select).selectpicker();
  // });
</script>

<select bind:this={select} class={classes} {multiple} on:change>
  {#each options as option}
    <option>{option}</option>
  {/each}
</select>
