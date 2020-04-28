<script>
  import Chart from "./Chart.svelte";
  import { activityChartConfig } from "../charts";
  import { getDaily } from "../gtm";
  import "chartjs-chart-matrix";

  export const commits = null;
  export let map = null;
</script>

<div class="row">
  <div class="col-12">
    <div style="width: 100%; height: 600px;">
      <Chart
        config={activityChartConfig(map.projects, getDaily(map.projects))} />
    </div>
  </div>
</div>
