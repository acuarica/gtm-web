<script>
  import ChartPalettePicker from "./ChartPalettePicker.svelte";

  export let navs;

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
  </div>
{/if}

<header class="bg-gray-700 sm:flex sm:justify-between">
  <div class="flex items-center justify-between px-4 py-1">
    <div>
      <img class="inline h-8" src="/assets/gtm-logo.png" alt="gtm Logo" />
      <span class="text-white">Dashboard</span>
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

    {#each navs as nav}
      <a
        href="{nav.href}"
        class="block sm:mx-2 mt-1 px-2 py-1 text-white rounded hover:bg-gray-800">
        {nav.title}
      </a>
    {/each}

    <button
      class="text-white px-2 py-1 mr-2 hidden sm:block focus:outline-none
      rounded hover:bg-gray-800"
      on:click={() => (toggleSettings = !toggleSettings)}
      type="button">
      <i class="fas fa-cog" />
    </button>

  </div>
</header>
