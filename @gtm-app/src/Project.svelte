<script>
  import { onMount } from "svelte";
  import Fetch from "./Fetch.svelte";
  import {
    timeByFileStatusChartConfig,
    projectTotalsChartConfig
  } from "./charts";
  import { hhmm } from "@gtm/format";
  import Chart from "./Chart.svelte";
  import DashboardCard from "./DashboardCard.svelte";
  import { activityChartConfig } from "./charts";
  import { computeWorkdirStatus } from "@gtm/notes";
  import "chartjs-chart-matrix";

  export let name;
  export let statsPromise;
  export let workdirStatsPromise;

  onMount(() => {});
</script>

<Fetch promise={statsPromise} let:value={res}>
  {#if res.stats.projects[name]}
    <div class="flex justify-around items-center">
      <DashboardCard
        title="Total Time"
        body={hhmm(res.stats.projects[name].total)}
        footer="Across {res.stats.projects[name].commitcount} commit{res.stats.projects[name].commitcount === 1 ? '' : 's'}" />

      <div class="w-64">
        <!-- <Chart config={timeByFileStatusChartConfig(res.stats.status)} /> -->
      </div>
    </div>

    <div>
      <Chart config={activityChartConfig([res.stats.projects[name]])} />
    </div>
  {:else}
    <p>No data in this period for project {name}.</p>
  {/if}
</Fetch>

<Fetch promise={workdirStatsPromise} let:value={res}>
  <div>
    <Chart config={activityChartConfig([res.projects[name]])} />
  </div>
</Fetch>
