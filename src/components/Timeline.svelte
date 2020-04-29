<script>
  import Chart from "./Chart.svelte";
  import { activityChartConfig } from "../charts";
  import { getDaily } from "../gtm";
  import "chartjs-chart-matrix";

  export const commits = null;
  export let map = null;
</script>

<div class="row12a">
  <div class="col-sdf12">
    <div style="width123: 100%; heightqwer : 600px;">
      <Chart
        config={activityChartConfig(map.projects, getDaily(map.projects))} />
    </div>
  </div>
</div>
