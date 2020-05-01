<script>
  import { onMount } from "svelte";
  import {
    timeByFileStatusChartConfig,
    projectTotalsChartConfig
  } from "./charts";
  import { hhmm } from "@gtm/format";
  import Chart from "./Chart.svelte";
  import Timeline from "./Timeline.svelte";
  import DashboardCard from "./DashboardCard.svelte";

  export let config;

  onMount(() => { });

  function timeByFileStatusChartDatasets() {
    const StatusIndicator = { m: "Modifying", r: "Reading", d: "Deleting" };

    return Object.keys(config.map.status).map(s => {
      return {
        data: [config.map.status[s]],
        label: `${StatusIndicator[s]}: ${hhmm(config.map.status[s])}`
      };
    });
  }

  function getds() {
    // const commitCounts: number[] = []
    const datasets = [];
    for (const pname in config.map.projects) {
      const p = config.map.projects[pname];
      // commitCounts.push(p.commitcount);
      datasets.push({
        data: [p.total],
        label: pname
      });
    }
    console.log(datasets);
    return datasets;
  }
</script>

<div class="flex justify-around items-center">
  <DashboardCard title="Total Time" body={hhmm(config.map.totalSecs)} footer="Across {config.commits.length} commits" />

  <div class="w-64">
    <Chart config={timeByFileStatusChartConfig()} datasets={timeByFileStatusChartDatasets()} />
  </div>
</div>

<div class="row">
  <div class="col-7">
    <div>
      <Chart config={projectTotalsChartConfig()} datasets={getds()} />
    </div>
    <div>
      <Timeline {config} />
    </div>
  </div>
</div>