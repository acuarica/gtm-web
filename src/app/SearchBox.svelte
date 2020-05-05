<script>
  import Icon from "./Icon.svelte";
  import { faSearch } from "@fortawesome/free-solid-svg-icons/faSearch";
</script>

<div class="bg-white flex items-center rounded-sm shadow-xl m-2">
  <input
    class="py-0 px-3 text-gray-700 focus:outline-none"
    type="text"
    placeholder="Search in commits ..." />
  <button class="text-blue-700 w-6 h-6 my-1 mx-2 focus:outline-none">
    <Icon class="mb-1 h-4" icon={faSearch} />
  </button>
</div>
