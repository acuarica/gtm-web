<script>
  import { onMount } from "svelte";
  import Chart from "./Chart.svelte";
  import { activityChartConfig } from "../charts";
  import { getDaily, computeWorkdirStatus } from "../gtm";
  import "chartjs-chart-matrix";

  export let config;

  function getWd(wd) {
    const wdmap = computeWorkdirStatus(wd);
    return activityChartConfig(wdmap.projects, getDaily(wdmap.projects));
  }
</script>

<div class="row12a">
  <div class="col-sdf12">
    <div style="width123: 100%; heightqwer : 600px;">
      <Chart
        config={activityChartConfig(config.map.projects, getDaily(config.map.projects))} />
    </div>
    {#await config.workdirStatus}
      <div>Waiting...</div>
    {:then wd}
      <div>
      asdf
        <Chart config={getWd(wd)} />
        ssadf
        1321
      </div>
    {/await}
  </div>
</div>
