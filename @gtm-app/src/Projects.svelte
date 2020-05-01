<script>
  import { onMount } from "svelte";
  import DashboardCard from "./components/DashboardCard.svelte";
  import { hhmm } from "@gtm/format";
  import Chart from "./components/Chart.svelte";
  import { activityChartConfig } from "./charts";
  import { getDaily, computeWorkdirStatus } from "@gtm/notes";
  import "chartjs-chart-matrix";

  export let config;

  let dsact;

  $: if (config.currentProject) dsact = getActivity(config.currentProject);
  // $: if (config.currentProject) dswd = getActivity();

  onMount(() => {
    console.log("project:mount", config.currentProject);
  });

  function getActivity(p) {
    const ds = [config.map.projects[p]];
    return activityChartConfig(ds, getDaily(ds));
  }

  function getWd(wd) {
    const wdmap = computeWorkdirStatus(wd);
    const ds = [wdmap.projects[config.currentProject]];
    return activityChartConfig(ds, getDaily(ds));
  }
</script>

<div>Projects</div>
<div>{config.currentProject}</div>

<DashboardCard
  title="Project Time"
  body={hhmm(config.map.projects[config.currentProject].total)}
  footer="Across - commits" />

<div class="row12a">
  <div class="col-sdf12">
    <div>
      <Chart config={dsact} />
    </div>
    {#await config.workdirStatus}
      <div>Waiting...</div>
    {:then wd}
      <div>
        <Chart config={getWd(wd)} />
      </div>
    {/await}
  </div>
</div>
