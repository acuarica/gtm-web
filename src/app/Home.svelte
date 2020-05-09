<script>
  import Box from "./Box.svelte";
  import { onMount } from "svelte";
  import Fetch from "./Fetch.svelte";
  import {
    activityChartConfig,
    timeByFileStatusChartConfig,
    projectTotalsChartConfig
  } from "../charts";
  import Chart from "./Chart.svelte";
  import DashboardCard from "./DashboardCard.svelte";
  import { hhmm } from "@gtm/format";
  import { computeWorkdirStatus } from "@gtm/notes";
  import "chartjs-chart-matrix";

  export let statsPromise;
  export let workdirStatsPromise;

  onMount(() => {});
</script>

<Fetch promise={statsPromise} let:value={res}>
  <div class="lg:flex">
    <div class="flex">
      <DashboardCard
        class="w-48 h-40 mr-3"
        title="Total Time"
        body={hhmm(res.stats.totalSecs)}
        footer="{res.commits.length} commit{res.commits.length === 1 ? '' : 's'}" />
      <Box class="flex-1 lg:w-64 h-40 pt-2 lg:mr-3">
        <Chart
          class="w-64"
          config={timeByFileStatusChartConfig(res.stats.status)} />
      </Box>
    </div>
    <Box class="flex justify-center mt-3 lg:mt-0 lg:flex-1 h-40">
      <Chart
        class="xxl:flex-1"
        style="width: 700px"
        config={projectTotalsChartConfig(res.stats.projects)} />
    </Box>
  </div>

  <!-- <div class="my-3"> -->
    <Box class="flex justify-center py-2 my-3">
      <Chart
        style="width: 94%; height: 400px"
        config={activityChartConfig(res.stats.projects)} />
    </Box>
  <!-- </div> -->
</Fetch>

<Fetch promise={workdirStatsPromise} let:value={res}>
  <Box class="flex justify-center py-2">
    <Chart
      style="width: 94%; height: 400px"
      config={activityChartConfig(res.projects, true)} />
  </Box>
</Fetch>
