<script>
  import { onMount } from "svelte";
  import Chart from "chart.js";
  import "chartjs-plugin-datalabels";
  export let config;
  export let datasets;
  let canvas;

  onMount(() => {
    const ctx = canvas.getContext("2d");
    console.log("@chart/mount",config)
    const chart = new Chart(ctx, config);

    console.log(datasets, "@chart");
    if (datasets) {
      chart.data.datasets = datasets;
      chart.update();
    }
  });
</script>

<canvas bind:this={canvas} />
