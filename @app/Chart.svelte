<script context="module">
  Chart.defaults.global.defaultFontColor = "#cbd5e0";
  Chart.defaults.global.defaultFontFamily = '"Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"';
  Chart.defaults.global.defaultFontSize = 14;
</script>

<script>
  import { onMount } from "svelte";
  import Chart from "chart.js";
  import "chartjs-plugin-datalabels";

  export let config;
  // export let datasets;

  let canvas;
  let chart;

  $: if (config && canvas) {
    if (chart) chart.destroy();
    // chart.config
    const ctx = canvas.getContext("2d");
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

<div class={$$props.class} style="flex: 0 0 auto; {$$props.style}">
  <canvas bind:this={canvas} />
</div>
