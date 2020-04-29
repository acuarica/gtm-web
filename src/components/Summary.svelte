<script>
  import { onMount } from "svelte";
  import {
    timeByFileStatusChartConfig,
    projectTotalsChartConfig
  } from "../charts";
  import { hhmm } from "./../gtm";
  import Chart from "./Chart.svelte";
  import Timeline from "./Timeline.svelte";

  export let config;

  onMount(() => {});

  function timeByFileStatusChartDatasets() {
    const StatusIndicator = { m: "Modify", r: "Read", d: "Delete" };

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

<div class="row">
  <div class="col-7">
    <div class="row">
      <div class="col-3">
        <div class="card">
          <div class="card-header">Total Time</div>
          <div class="card-body">
            <h5 class="card-title text-primary">
              {hhmm(config.map.totalSecs)}
            </h5>
            <p class="card-text text-muted">
              In
              <span class="text-info">{config.commits.length}</span>
              commits
            </p>
          </div>
        </div>
      </div>
      <div class="col-6">
        <div style="width: 100%;">
          <Chart
            config={timeByFileStatusChartConfig()}
            datasets={timeByFileStatusChartDatasets()} />
        </div>
      </div>
    </div>
    <div style="width: 100%;">
      <Chart config={projectTotalsChartConfig()} datasets={getds()} />
    </div>
    <div style="width: 100%;">
      <Timeline {config} />
    </div>
  </div>
</div>
