<script>
  import { onMount } from "svelte";
  import Fetch from "./Fetch.svelte";
  import {
    activityChartConfig,
    timeByFileStatusChartConfig,
    projectTotalsChartConfig
  } from "../charts";
  import { hhmm } from "../format";
  import Chart from "./Chart.svelte";
  import DashboardCard from "./DashboardCard.svelte";
  import { computeWorkdirStatus } from "../notes";
  import "chartjs-chart-matrix";

  export let statsPromise;
  export let workdirStatsPromise;

  onMount(() => {});
</script>

<Fetch promise={statsPromise} let:value={res}>
  <div class="flex justify-around items-center">
    <DashboardCard
      title="Total Time"
      body={hhmm(res.stats.totalSecs)}
      footer="{res.commits.length} commit{res.commits.length === 1 ? '' : 's'}" />

    <div class="w-64">
      <Chart config={timeByFileStatusChartConfig(res.stats.status)} />
    </div>

    <div class="w-1/2">
      <Chart config={projectTotalsChartConfig(res.stats.projects)} />
    </div>
  </div>

  <div class="mx-6">
    <Chart config={activityChartConfig(res.stats.projects)} />
  </div>
</Fetch>

<Fetch promise={workdirStatsPromise} let:value={res}>
  <div class="mx-6">
    <Chart config={activityChartConfig(res.projects)} />
  </div>
</Fetch>
