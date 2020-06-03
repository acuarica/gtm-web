<script>
  import "./main.pcss";
  import Box from "./Box.svelte";
  import Icon from "./Icon.svelte";
  import { faTasks } from "@fortawesome/free-solid-svg-icons/faTasks";
  import { onMount } from "svelte";
  import { computeStats, computeWorkdirStatus } from "@gtm/notes";
  import Fetch from "./Fetch.svelte";
  import Navbar from "./Navbar.svelte";
  import Progress from "./Progress.svelte";
  import Select from "./Select.svelte";
  import Home from "./Home.svelte";
  import Project from "./Project.svelte";
  import Commits from "./Commits.svelte";
  import DashboardCard from "./DashboardCard.svelte";

  export let fetchCommits;
  export let fetchProjectList;
  export let fetchWorkdirStatus;
  export let settingsView;
  export let settingsViewProps;

  let statsPromise = new Promise((_resolve, _reject) => {});
  let projectListPromise = new Promise((_resolve, _reject) => {});
  let workdirStatsPromise = new Promise((_resolve, _reject) => {});
  let title;
  let view = Home;
  let viewProject;
  let params;

  $: if (view === Home) {
    title = "All Projects";
    params = {
      statsPromise: statsPromise,
      workdirStatsPromise: workdirStatsPromise
    };
  }

  $: if (view === Project) {
    title = viewProject;
    params = {
      name: viewProject,
      projectPromise: statsPromise.then(s => s.projects[viewProject]),
      workdirStatsPromise: workdirStatsPromise.then(
        s => s.projects[viewProject]
      )
    };
  }

  onMount(() => {
    projectListPromise = fetchProjectList();
    workdirStatsPromise = fetchWorkdirStatus().then(ws =>
      computeWorkdirStatus(ws)
    );
  });

  let currentFilter = {};

  function handleRangeChange(event) {
    fetchStats(event.detail);
  }

  function handleSearch(event) {
    fetchStats({ message: event.detail.text });
  }

  function fetchStats(partialFilter) {
    currentFilter = { ...currentFilter, ...partialFilter };
    statsPromise = fetchCommits(currentFilter).then(cs => computeStats(cs));
  }
</script>

<div class="container antialiased serif h-screen">
  <div class="flex flex-col h-full">

    <Navbar
      {title}
      {handleRangeChange}
      {settingsView}
      {settingsViewProps}
      on:search={handleSearch} />

    <div class="flex flex-1 ">
      <div class="flex flex-row w-full divide-x divide-divide-color">
        <div class="hidden sm:block bg-sidebar p-3">

          <Box class="w-56 flex-shrink-0 p-3 h-full">
            {#await projectListPromise}
              <!-- Only to linting complain -->
            {:then projectList}
              <button
                class="block py-1 pl-1 text-lg rounded hover:bg-gray-600
                hover:text-gray-300"
                on:click={() => {
                  view = Home;
                }}>
                <Icon class="mb-1 h-4" icon={faTasks} />
                <span class={view === Home ? 'font-bold' : ''}>
                  All Projects
                </span>
              </button>

              {#each projectList as project}
                <button
                  class="block py-1 px-6 rounded hover:bg-gray-600
                  hover:text-gray-300 {view === Project && viewProject === project ? 'font-bold' : ''}"
                  on:click={() => {
                    viewProject = project;
                    view = Project;
                  }}>
                  {project}
                </button>
              {/each}
            {/await}
          </Box>

        </div>

        <div class="bg-view p-3 flex-1 w-auto flex-col">
          {#await projectListPromise}
            <Progress />
          {:then _projectList}
            <svelte:component this={view} {...params} />
          {:catch error}
            <Box class="h-full p-4">
              <DashboardCard
                title="Could not get gtm projects"
                body="Make sure gtm is installed"
                footer="Got error: {error}" />
            </Box>
          {/await}
        </div>

      </div>
    </div>
  </div>
</div>
