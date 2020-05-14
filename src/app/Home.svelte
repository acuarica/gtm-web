<script>
  import Box from "./Box.svelte";
  import { onMount } from "svelte";
  import Fetch from "./Fetch.svelte";
  import {
    activityChartConfig,
    timeByFileStatusChartConfig,
    projectTotalsChartConfig
  } from "./charts";
  import Chart from "./Chart.svelte";
  import DashboardCard from "./DashboardCard.svelte";
  import { computeWorkdirStatus, hhmm } from "@gtm/notes";
  import "chartjs-chart-matrix";

  export let statsPromise;
  export let workdirStatsPromise;

  console.assert(statsPromise, "Stats promise cannot be null");
  console.assert(workdirStatsPromise, "Workdir stats promise cannot be null");

  onMount(() => {});
</script>

<div class="grid grid-cols-12 gap-3">
  <Fetch promise={statsPromise} let:value={stats}>
    <DashboardCard
      class="col-span-12 md:col-span-5 lg:col-span-3 xl:col-span-2"
      title="Total Time"
      body={hhmm(stats.totalSecs)}
      footer="{stats.commitCount} commit{stats.commitCount === 1 ? '' : 's'}" />
    <Box class="col-span-12 md:col-span-7 lg:col-span-4 xl:col-span-3">
      <Chart config={timeByFileStatusChartConfig(stats.status)} />
    </Box>
    <Box class="col-span-12 lg:col-span-5 xl:col-span-7">
      <Chart config={projectTotalsChartConfig(stats.projects)} />
    </Box>
    <Box class="col-span-12">
      <Chart
        style="height: 400px"
        config={activityChartConfig(stats.projects)} />
    </Box>
  </Fetch>
  <Fetch promise={workdirStatsPromise} let:value={workdirStats}>
    <Box class="col-span-12">
      <Chart
        style="height: 400px"
        config={activityChartConfig(workdirStats.projects, true)} />
    </Box>
  </Fetch>
</div>
