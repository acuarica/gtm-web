<script>
  import Icon from "./Icon.svelte";
  import { faTasks } from "@fortawesome/free-solid-svg-icons/faTasks";
  import { faCog } from "@fortawesome/free-solid-svg-icons/faCog";
  import SearchBox from "./SearchBox.svelte";
  import DatePicker from "./DatePicker.svelte";
  import DateRangePicker from "./DateRangePicker.svelte";
  import ChartPalettePicker from "./ChartPalettePicker.svelte";

  import logo from "../../assets/gtm-logo.png";
  export let title = "";
  export let handleRangeChange;
  export let settingsView;

  let isOpen = false;
  let toggleSettings = false;
</script>

{#if toggleSettings}
  <div class="bg-gray-600 p-4">
    <h4 class="text-white">Settings</h4>
    <form>
      <div class="form-group2">
        <label class="text-light">Color Pallette for Charts</label>
        <ChartPalettePicker classes="form-control4" />
      </div>
    </form>
    <svelte:component this={settingsView} />
  </div>
{/if}

<header class="bg-navbar sm:flex sm:justify-between">
  <div class="flex items-center justify-between px-4 py-1">
    <div>
      <a href="/">
        <img class="inline h-8" src={logo} alt="gtm Logo" />
        <span class="text-white">Dashboard</span>
      </a>
      <span class="ml-12 font-medium text-white">
        <Icon class="mr-1 mb-1 h-4" icon={faTasks} />
        {title}
      </span>
    </div>
    <div class="sm:hidden">
      <button
        on:click={() => (isOpen = !isOpen)}
        type="button"
        class="block text-gray-400 hover:text-white focus:text-white
        focus:outline-none">
        <svg class="h-6 w-6 fill-current" viewBox="0 0 24 24">
          {#if isOpen}
            <path
              fill-rule="evenodd"
              d="M18.278 16.864a1 1 0 0 1-1.414 1.414l-4.829-4.828-4.828 4.828a1
              1 0 0 1-1.414-1.414l4.828-4.829-4.828-4.828a1 1 0 0 1
              1.414-1.414l4.829 4.828 4.828-4.828a1 1 0 1 1 1.414 1.414l-4.828
              4.829 4.828 4.828z" />
          {:else}
            <path
              fill-rule="evenodd"
              d="M4 5h16a1 1 0 0 1 0 2H4a1 1 0 1 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1
              1 0 0 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2z" />
          {/if}
        </svg>
      </button>
    </div>
  </div>
  <div class="px-2 pb-1 sm:flex sm:items-center {isOpen ? 'block' : 'hidden'}">

    <DatePicker on:change={handleRangeChange} />
    <SearchBox />
    <button
      class="text-white px-2 py-1 mr-2 hidden sm:block focus:outline-none
      rounded hover:bg-gray-800"
      on:click={() => (toggleSettings = !toggleSettings)}
      type="button">
      <Icon class="mb-1 h-4" icon={faCog} />
    </button>

  </div>
</header>
