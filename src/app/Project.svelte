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
  import { computeWorkdirStatus, hhmm } from "@gtm/notes";
  import "chartjs-chart-matrix";

  export let name;
  export let projectPromise;
  export let workdirStatsPromise;

  function cardFooterText(commits) {
    return `${commits.length} commit${commits.length === 1 ? "" : "s"}`;
  }

  onMount(() => {});
</script>

<Fetch promise={projectPromise} let:value={project}>
  {#if project}
    <div class="grid grid-cols-12 gap-3">
      <DashboardCard
        class="col-span-6"
        title="Total Time"
        body={hhmm(project.total)}
        footer={cardFooterText(project.commits)} 
        />
      <Box class="col-span-6">
        <Chart
          config={timeByFileStatusChartConfig(project.status)} />
      </Box>
      <Box class="col-span-12 px-6 py-3">
        <Chart config={activityChartConfig([project])} />
      </Box>
      <Box class="col-span-12 px-6 py-3">
        <Fetch promise={workdirStatsPromise} let:value={workdirStats}>
          <Chart config={activityChartConfig([workdirStats], true)} />
        </Fetch>
      </Box>
      <Box class="col-span-12 lg:col-span-6 p-4">
        <div class="font-bold">Files</div>
        <FileNotes files={project.files} />
      </Box>
      <Commits
        class="col-span-12 lg:col-span-6"
        commits={project.commits} />
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
