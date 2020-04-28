<script>
  import { onMount, setContext } from "svelte";
  import { computeStats } from "./gtm";
  import Navbar from "./components/Navbar.svelte";
  import Progress from "./components/Progress.svelte";
  import Select from "./components/Select.svelte";
  import DateRangePicker from "./components/DateRangePicker.svelte";
  import Summary from "./components/Summary.svelte";
  import Projects from "./components/Projects.svelte";
  import Timeline from "./components/Timeline.svelte";
  import Commits from "./components/Commits.svelte";

  export let fetchCommits;
  export let fetchProjectList;
  export const fetchWorkDirStatus = null;

  const navs = [
    { title: "Summary", view: Summary },
    { title: "Projects", view: Projects },
    { title: "Timeline", view: Timeline },
    { title: "Commits", view: Commits }
  ];

  let view = Summary;
  let promise = new Promise((_resolve, _reject) => {});
  let projectList = [];

  onMount(async () => {
    projectList = await fetchProjectList();
  });

  function handleRangeChange(event) {
    console.log("handle range");
    console.log(event.detail);
    promise = fetchCommits(event.detail);
  }
</script>

<Navbar />

<nav class="navbar navbar-expand-lg navbar-dark bg-dark">
  <div class="collapse navbar-collapse" id="main-navbar">
    <ul class="navbar-nav mr-auto">
      <li class="nav-item active">
        <a class="nav-link" href="#3">
          <i class="fas fa-home" />
        </a>
      </li>
      {#each navs as nav}
        <li class="nav-item">
          <button
            type="button"
            class="nav-link btn btn-link"
            on:click={() => (view = nav.view)}>
            {nav.title}
          </button>
        </li>
      {/each}
    </ul>
    <DateRangePicker on:change={handleRangeChange} />
    <form class="form-inline my-2 my-md-0">
      {#if projectList.length > 0}
        <Select options={projectList} multiple />
      {/if}
      <input
        class="form-control mr-sm-2 form-control-sm"
        type="text"
        placeholder="Search in commits ..." />
    </form>
  </div>
</nav>

{#await promise}
  <Progress />
{:then commits}
  <div class="container-fluid">
    <svelte:component this={view} {commits} map={computeStats(commits)} />
  </div>
{:catch error}
  <p style="color: red">{error.message}</p>
{/await}
