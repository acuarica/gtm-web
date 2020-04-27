<script>
  import { onMount, setContext } from "svelte";
  import { computeStats } from "./gtm";
  import Settings from "./components/Settings.svelte";
  import Progress from "./components/Progress.svelte";
  import DateRangePicker from "./components/DateRangePicker.svelte";
  import Summary from "./components/Summary.svelte";
  import Projects from "./components/Projects.svelte";
  import Timeline from "./components/Timeline.svelte";
  import Commits from "./components/Commits.svelte";

  export let view = Summary;
  export let toggleSettings = false;

  const navs = [
    { title: "Summary", view: Summary },
    { title: "Projects", view: Projects },
    { title: "Timeline", view: Timeline },
    { title: "Commits", view: Commits }
  ];

  let promise = new Promise((_resolve, _reject) => {
    console.log("init");
  }); //= fetchCommits();

  // Chart.defaults.global.plugins.colorschemes.scheme = "tableau.Tableau10";

  async function fetchCommits(range) {
    const commitsDataUrl = "/data/commits";
    const url = `${commitsDataUrl}?all&from=${range.start}&to=${range.end}`;
    const json = await fetch(url).then(r => r.json());
    return json;
  }

  function handleRangeChange(event) {
    console.log("handle range");
    console.log(event.detail);
    promise = fetchCommits(event.detail);
  }
</script>

{#if toggleSettings}
  <Settings />
{/if}

<nav class="navbar navbar-expand-lg navbar-dark bg-dark">
  <a class="navbar-brand" href="#">
    <img src="../assets/gtm-logo.png" width="82" height="34" alt="gtm Logo" />
    Dashboard
  </a>
  <button
    class="navbar-toggler"
    type="button"
    data-toggle="collapse"
    data-target="#main-navbar"
    aria-controls="main-navbar"
    aria-expanded="false"
    aria-label="Toggle navigation">
    <span class="navbar-toggler-icon" />
  </button>
  <div class="collapse navbar-collapse" id="main-navbar">
    <ul class="navbar-nav mr-auto">
      <li class="nav-item active">
        <a class="nav-link" href="#">
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
    <!-- <DateRangePicker
      on:change={event => (promise = fetchCommits(event.detail))} /> -->
    <form class="form-inline my-2 my-md-0">
      <input
        class="form-control mr-sm-2 form-control-sm"
        type="text"
        placeholder="Search in commits ..." />
      <button
        class="btn btn-outline-secondary btn-sm"
        on:click={() => (toggleSettings = !toggleSettings)}
        type="button">
        <i class="fas fa-cog" />
      </button>
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
