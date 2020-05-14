<script>
  import Box from "./Box.svelte";
  import Icon from "./Icon.svelte";
  import { faTasks } from "@fortawesome/free-solid-svg-icons/faTasks";
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
  export let settingsView;

  let statsPromise = new Promise((_resolve, _reject) => {});
  let projectListPromise = new Promise((_resolve, _reject) => {});
  let workdirStatsPromise = new Promise((_resolve, _reject) => {});
  let projectName;
  let title;
  let view = Home;

  function getBase() {
    const pathname = window.location.pathname;
    const parts = pathname.split("/");
    console.assert(parts.length >= 2, "not / found");
    console.assert(parts[0] === "", "trailing string before /");
    parts.shift();
    if (parts[parts.length - 1] === "") {
      parts.pop();
    }
    const base = "/" + parts.join("/");
    console.info("Using base path:", base);
    return base;
  }

  router.base(getBase());
  router("/", () => {
    title = "All Projects";
    view = Home;
  });
  router("/projects/:project", ctx => {
    projectName = ctx.params.project;
    title = projectName;
    view = Project;
  });

  onMount(async () => {
    projectListPromise = fetchProjectList();
    const workdirStatus = await fetchWorkdirStatus();
    workdirStatsPromise = Promise.resolve(computeWorkdirStatus(workdirStatus));
    router({ hashbang: true });
  });

  async function fetch(event) {
    const commits = await fetchCommits(event.detail);
    return Promise.resolve(computeStats(commits));
  }

  function handleRangeChange(event) {
    statsPromise = fetch(event);
  }
</script>

<div class="container antialiased serif h-screen">
  <div class="flex flex-col h-full divide-y divide-divide-color">

    <Navbar {title} {handleRangeChange} {settingsView} />

    <div class="flex flex-1 ">
      <div class="flex flex-row w-full divide-x divide-divide-color">
        <div class="hidden sm:block bg-sidebar p-3">

          <Box class="w-56 flex-shrink-0 p-3 h-full">
            <a
              class="block py-1 pl-1 text-lg rounded hover:bg-gray-600
              hover:text-gray-300"
              href="./">
              <Icon class="mb-1 h-4" icon={faTasks} />
              <span class={view === Home ? 'font-bold' : ''}>All Projects</span>
            </a>

            <Fetch promise={projectListPromise} let:value={projectList}>
              {#each projectList as project}
                <a
                  class="block py-1 pl-6 rounded hover:bg-gray-600
                  hover:text-gray-300 {view === Project && projectName === project ? 'font-bold' : ''}"
                  href="./projects/{project}">
                  {project}
                </a>
              {/each}
            </Fetch>
          </Box>

        </div>
        <div class="bg-view p-3 flex-1 w-auto flex-col">
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
