<script>
  import { onMount, setContext } from "svelte";
  import { fly, slide } from "svelte/transition";
  import { computeStats } from "./gtm";
  import router from "page";
  import Navbar from "./components/Navbar.svelte";
  import Progress from "./components/Progress.svelte";
  import Select from "./components/Select.svelte";
  import Summary from "./components/Summary.svelte";
  import Projects from "./components/Projects.svelte";
  import Timeline from "./components/Timeline.svelte";
  import Commits from "./components/Commits.svelte";

  export let fetchCommits;
  export let fetchProjectList;
  export let fetchWorkdirStatus;

  const navs = [
    { title: "Working Trees", view: Summary, href: "/" },
    { title: "Timeline", view: Timeline, href: "/timeline" },
    { title: "Projects", view: Projects, href: "/projects" },
    { title: "Commits", view: Commits, href: "/commits" }
  ];

  let view = Summary;
  let promise = new Promise((_resolve, _reject) => {});
  let projectList = [];
  let workdirStatus;

  let toggleProjects = true;

  for (const nav of navs) {
    router(nav.href, () => (view = nav.view));
  }
  router.start();

  onMount(async () => {
    projectList = await fetchProjectList();
    workdirStatus = fetchWorkdirStatus();
  });

  function handleRangeChange(event) {
    console.log("handle range");
    console.log(event.detail);
    promise = fetchCommits(event.detail);
  }

  function getConfig(commits) {
    return {
      commits: commits,
      map: computeStats(commits),
      projectList: projectList,
      workdirStatus: workdirStatus
    };
  }
</script>

<div class="antialiased sans-serif h-screen">
  <div class="flex flex-col h-full">

    <Navbar {navs} {handleRangeChange} />

    <div class="flex flex-1 w-screen">
      <div class="flex flex-row w-full">
        <div class="w-56 flex-shrink-0 bg-gray-500 p-3">
          <button
            type="button"
            class="w-5 focus:outline-none"
            on:click={() => (toggleProjects = !toggleProjects)}>
            {#if toggleProjects}
              <i class="fas fa-chevron-down" />
            {:else}
              <i class="fas fa-chevron-right" />
            {/if}
          </button>

          <a class="text-lg rounded hover:bg-gray-400" href="/projects">
            Projects
          </a>

          {#if toggleProjects}
            <div transition:slide={{ delay: 0, duration: 100 }}>
              {#each projectList as project}
                <a
                  class="block py-1 pl-6 rounded hover:bg-gray-400"
                  href="/projects/{project}">
                  {project}
                </a>
              {/each}
            </div>
          {/if}
        </div>

        <div class="flex-1 w-auto flex-col bg-blue-100">
          {#await promise}
            <Progress />
          {:then commits}
            <div>
              <svelte:component this={view} config={getConfig(commits)} />
            </div>
          {:catch error}
            <p style="color: red">{error.message}</p>
          {/await}
        </div>

      </div>
    </div>
  </div>
</div>
