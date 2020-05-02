<script>
  import { onMount } from "svelte";
  import { computeStats, computeWorkdirStatus } from "@gtm/notes";
  import router from "page";
  import Fetch from "./Fetch.svelte";
  import Navbar from "./Navbar.svelte";
  import Progress from "./Progress.svelte";
  import Select from "./Select.svelte";
  import Home from "./Home.svelte";
  import Project from "./Project.svelte";
  import Commits from "./Commits.svelte";

  export let fetchCommits;
  export let fetchProjectList;
  export let fetchWorkdirStatus;

  let statsPromise = new Promise((_resolve, _reject) => {});
  let projectListPromise = new Promise((_resolve, _reject) => {});
  let workdirStatsPromise = new Promise((_resolve, _reject) => {});
  let projectName;
  let view;

  router("/", async () => {
    view = Home;
  });
  router("/projects/:project", ctx => {
    projectName = ctx.params.project;
    view = Project;
  });

  onMount(async () => {
    router.start();
    projectListPromise = fetchProjectList();
    const workdirStatus = await fetchWorkdirStatus();
    workdirStatsPromise = Promise.resolve(computeWorkdirStatus(workdirStatus));
    console.log(workdirStatsPromise);
  });

  async function fetch(event) {
    const commits = await fetchCommits(event.detail);
    return Promise.resolve({
      commits: commits,
      stats: computeStats(commits)
    });
  }

  function handleRangeChange(event) {
    statsPromise = fetch(event);
  }
</script>

<div class="antialiased sans-serif h-screen">
  <div class="flex flex-col h-full">

    <Navbar {handleRangeChange} />

    <div class="flex flex-1 w-screen">
      <div class="flex flex-row w-full">
        <div class="w-56 flex-shrink-0 bg-gray-500 p-3">

          <a
            class="block py-1 pl-1 text-lg rounded hover:bg-gray-600
            hover:text-gray-300"
            href="/">
            <i class="fas fa-tasks" />
            All Projects
          </a>

          <Fetch promise={projectListPromise} let:value={projectList}>
            {#each projectList as project}
              <a
                class="block py-1 pl-6 rounded hover:bg-gray-600
                hover:text-gray-300"
                href="/projects/{project}">
                {project}
              </a>
            {/each}
          </Fetch>
        </div>

        <div class="flex-1 w-auto flex-col bg-gray-200">
          <div>
            <svelte:component
              this={view}
              {statsPromise}
              {workdirStatsPromise}
              name={projectName} />
          </div>
        </div>

      </div>
    </div>
  </div>
</div>
