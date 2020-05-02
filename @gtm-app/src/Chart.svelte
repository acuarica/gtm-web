<script>
  import { onMount } from "svelte";
  import Chart from "chart.js";
  import "chartjs-plugin-datalabels";

  export let config;
  // export let datasets;

  let canvas;
  let chart;

  $: if (config &&canvas) {
    if (chart)
    chart.destroy()
    // chart.config 
    const ctx = canvas.getContext("2d");
    console.log("@chart/mount", config);
    chart = new Chart(ctx, config);
    // chart.data = config.data
    // chart.options = config.options
    // chart.update()
  }

  // function create(config) {
  //   const ctx = canvas.getContext("2d");
  //   console.log("@chart/mount", config);
  //   return new Chart(ctx, config);


  // }


  onMount(() => {
    // const ctx = canvas.getContext("2d");
    // console.log("@chart/mount", config);
    // chart = new Chart(ctx, config);

    // console.log(datasets, "@chart");
    // if (datasets) {
    //   chart.data.datasets = datasets;
    //   chart.update();
    // }
  });
</script>

<canvas bind:this={canvas} />
