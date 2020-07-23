<script>
  import ChartPalettePicker from "./ChartPalettePicker.svelte";

  export let versions;

  console.assert(versions, "Settings `versions` cannot be null or undefined");
</script>

<form>
  <div class="form-group2">
    <label class="text-light">Color Pallette for Charts</label>
    <ChartPalettePicker classes="form-control4" />
  </div>
</form>

<h4 class="text-white">About</h4>
{#each Object.keys(versions) as version}
  <div>
    {version}
    <span id="node-version">{versions[version]}</span>
  </div>
{/each}
