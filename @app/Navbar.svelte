<script>
  import Box from "./Box.svelte";
  import Icon from "./Icon.svelte";
  import { faTasks } from "@fortawesome/free-solid-svg-icons/faTasks";
  import { faCog } from "@fortawesome/free-solid-svg-icons/faCog";
  import { faAngleRight } from "@fortawesome/free-solid-svg-icons/faAngleRight";
  import SearchBox from "./SearchBox.svelte";
  import DatePicker from "./DatePicker.svelte";
  import DateRangePicker from "./DateRangePicker.svelte";

  import logo from "./assets/gtm-logo.png";
  export let title = "";
  export let handleRangeChange;
  export let settingsView;
  export let settingsViewProps;

  let isOpen = false;
  let toggleSettings = false;
</script>

{#if toggleSettings}
  <div class="bg-gray-600 p-4">
    <h3 class="text-white">Settings</h3>
    <svelte:component this={settingsView} {...settingsViewProps} />
  </div>
{/if}

<div class="bg-sidebar text-sm">
  <Box class="md:flex md:justify-between mx-3">
    <div class="flex items-center justify-between px-4 py-1">
      <div>
        <a href="./">
          <img class="inline h-8" src={logo} alt="gtm Logo" />
          <span class="text-white">Dashboard</span>
        </a>
        <span class="ml-2 font-medium text-white">
          <Icon class="mr-2 mb-1 h-4" icon={faAngleRight} />
          {title}
        </span>
      </div>
      <div class="md:hidden">
        <button
          on:click={() => (isOpen = !isOpen)}
          type="button"
          class="block text-gray-400 hover:text-white focus:text-white
          focus:outline-none">
          <svg class="h-6 w-6 fill-current" viewBox="0 0 24 24">
            {#if isOpen}
              <path
                fill-rule="evenodd"
                d="M18.278 16.864a1 1 0 0 1-1.414 1.414l-4.829-4.828-4.828
                4.828a1 1 0 0 1-1.414-1.414l4.828-4.829-4.828-4.828a1 1 0 0 1
                1.414-1.414l4.829 4.828 4.828-4.828a1 1 0 1 1 1.414 1.414l-4.828
                4.829 4.828 4.828z" />
            {:else}
              <path
                fill-rule="evenodd"
                d="M4 5h16a1 1 0 0 1 0 2H4a1 1 0 1 1 0-2zm0 6h16a1 1 0 0 1 0
                2H4a1 1 0 0 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2z" />
            {/if}
          </svg>
        </button>
      </div>
    </div>
    <div
      class="px-2 pb-1 md:flex md:items-center {isOpen ? 'block' : 'hidden'}">
      <div class="mr-2 hidden lg:block text-xs">Commit Filters</div>
      <DatePicker on:change={handleRangeChange} />
      <SearchBox on:search />
      <button
        class="text-white px-2 py-1 mr-2 hidden sm:block focus:outline-none
        rounded hover:bg-gray-800"
        on:click={() => (toggleSettings = !toggleSettings)}
        type="button">
        <Icon class="mb-1 h-4" icon={faCog} />
      </button>

    </div>
  </Box>

</div>
