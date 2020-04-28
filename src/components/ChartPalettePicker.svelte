<script>
  import { onMount } from "svelte";
  import Chart from "chart.js";
  // import * as Chart from "chart.js";
  // import * as JQ from "jquery";
  // import "bootstrap";
  // import "bootstrap-select";
  import "chartjs-plugin-colorschemes";

  export let palette = [
    "tableau.Tableau10",
    "office.Excel16",
    "tableau.Tableau20",
    "tableau.Classic10",
    "tableau.ColorBlind10"
  ];
  export let classes = "";

  let select;

  onMount(() => {
    // JQ(select).selectpicker();
  });

  const options = palette.map(e => {
    const [group, pallete] = e.split(".");
    const colorSchemes = Chart.colorschemes;
    return {
      value: e,
      data: {
        "data-width": "400px",
        "data-content": `<div><div style="width: 80px; display: inline-block"><small class="text-muted">${pallete}</small></div>
          ${colorSchemes[group][pallete]
            .map(
              color =>
                `<div style="background-color: ${color}; width: 12px; display: inline-block">&nbsp;</div>`
            )
            .join("")}
          </div>`
      }
    };
  });

  function handleChange() {
    Chart.defaults.global.plugins.colorschemes.scheme = select.value;
    for (const i in Chart.instances) {
      const chart = Chart.instances[i];
      chart.options.plugins.colorschemes.scheme = select.value;
      chart.update();
    }
  }
</script>

<select bind:this={select} class={classes} on:change={handleChange}>
  {#each options as option}
    <option {...option.data}>{option.value}</option>
  {/each}
</select>
