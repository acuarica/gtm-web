<script>
  import { onMount, setContext } from "svelte";
  import { computeStats } from "@gtm/notes";
  import router from "page";
  import Navbar from "./Navbar.svelte";
  import Progress from "./Progress.svelte";
  import Select from "./Select.svelte";
  import Summary from "./Summary.svelte";
  import Projects from "./Projects.svelte";
  import Timeline from "./Timeline.svelte";
  import Commits from "./Commits.svelte";

  export let fetchCommits;
  export let fetchProjectList;
  export let fetchWorkdirStatus;

  let view = Summary;
  let promise = new Promise((_resolve, _reject) => {});
  let projectList = [];
  let workdirStatus;
  let currentProject;
  let config;

  $: if (config) config.currentProject = currentProject;
  let toggleProjects = true;

  router("/", () => (view = Summary));

  let views = {}

  $: if (config&&views&&currentProject) view = views[currentProject].view

  router("/projects/:project", ctx => {
    console.log(Projects);
    console.log(view);
    console.log(ctx);
    // Projects.projectName = "asdfasdf"

    // Projects;
    // Projects.$set({ projectName: "asdsa23232" });
    // view.$set

    let b  = views[currentProject]===views[ctx.params.project]
    console.log(b, "equal?")

    // console.log(currentProject);
    currentProject = ctx.params.project;
    // view = Projects;
    view = null
    view = views[currentProject].view
    // view = view;
    config = config;

    // new Projects({
    //   target: view,
    //   props: { projectName:
    // });
  });

  router.start();

  onMount(async () => {
    projectList = await fetchProjectList();
    workdirStatus = fetchWorkdirStatus();

    for (const pkey of projectList) {
      views[pkey] = {view:Projects}
    }
  });

  function handleRangeChange(event) {
    console.log("handle range");
    console.log(event.detail);
    promise = fetchCommits(event.detail);
    promise.then(cs => (config = getConfig(cs)));
  }

  function getConfig(commits) {
    console.log(currentProject, "@getconfig");
    return {
      commits: commits,
      map: computeStats(commits),
      projectList: projectList,
      workdirStatus: workdirStatus,
      currentProject: currentProject
    };
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

          {#each projectList as project}
            <a
              class="block py-1 pl-6 rounded hover:bg-gray-600
              hover:text-gray-300"
              href="/projects/{project}">
              {project}
            </a>
          {/each}
        </div>

        <div class="flex-1 w-auto flex-col bg-gray-200">
          {#await promise}
            <Progress />
          {:then commits}
            <div>
              <svelte:component this={view} {config} />
            </div>
          {:catch error}
            <p style="color: red">{error.message}</p>
          {/await}
        </div>

      </div>
    </div>
  </div>
</div>
