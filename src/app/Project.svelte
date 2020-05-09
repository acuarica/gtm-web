<script>
  import { onMount } from "svelte";
  import Box from "./Box.svelte";
  import Fetch from "./Fetch.svelte";
  import {
    activityChartConfig,
    timeByFileStatusChartConfig,
    projectTotalsChartConfig
  } from "../charts";
  import Chart from "./Chart.svelte";
  import Commits from "./Commits.svelte";
  import DashboardCard from "./DashboardCard.svelte";
  import FileNotes from "./FileNotes.svelte";
  import { hhmm } from "@gtm/format";
  import { computeWorkdirStatus } from "@gtm/notes";
  import "chartjs-chart-matrix";

  export let name;
  export let statsPromise;
  export let workdirStatsPromise;

  function cardFooterText(commits) {
    return `${commits.length} commit${commits.length === 1 ? "" : "s"}`;
  }

  onMount(() => {});
</script>

<Fetch promise={statsPromise} let:value={res}>
  {#if res.stats.projects[name]}
    <div class="flex justify-around items-center">
      <DashboardCard
        title="Total Time"
        body={hhmm(res.stats.projects[name].total)}
        footer={cardFooterText(res.stats.projects[name].commits)} />

      <div class="w-64">
        <Box>
          <Chart
            config={timeByFileStatusChartConfig(res.stats.projects[name].status)} />
        </Box>
      </div>
    </div>

    <div class="my-3">
      <Box class="flex justify-center py-2">
        <Chart
          style="width: 94%; height: 400px"
          config={activityChartConfig([res.stats.projects[name]])} />
      </Box>

      <Fetch promise={workdirStatsPromise} let:value={res}>
        <Box class="mt-3 p-3">
          <Chart config={activityChartConfig([res.projects[name]], true)} />
        </Box>
      </Fetch>

    </div>
    <div class="grid grid-cols-2 col-gap-2">
      <Box class="p-3" >
        <div class="font-bold">Files</div>
        <FileNotes files={res.stats.projects[name].files} />
      </Box>
      <Commits class="" commits={res.stats.projects[name].commits} />
    </div>
  {:else}
    <Box class="p-8">
      <div class="text-lg">
        No time data was found in this period for project
        <span class="text-highlight font-bold">{name}</span>
        .
      </div>
      <div class="text-sm text-muted mt-3">
        There are no commits during this time period. Try another date range.
      </div>
    </Box>
  {/if}
</Fetch>
