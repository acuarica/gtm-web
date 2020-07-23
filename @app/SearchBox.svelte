<script>
  import Icon from "./Icon.svelte";
  import { faSearch } from "@fortawesome/free-solid-svg-icons/faSearch";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  let text = "";

  function onClick() {
    dispatch("search", {
      text: text
    });
  }

  function handleKeydown(event) {
    if (event.keyCode === 13) {
      dispatch("search", {
        text: text
      });
    }
  }
</script>

<div class="bg-white flex items-center rounded-sm shadow-xl m-2">
  <input
    bind:value={text}
    class="py-0 px-3 text-gray-700 focus:outline-none w-40"
    type="text"
    placeholder="Messages ..."
    on:keydown={handleKeydown} />
  <button
    class="text-blue-700 w-6 h-6 my-1 mx-2 focus:outline-none"
    on:click={onClick}>
    <Icon class="mb-1 h-4" icon={faSearch} />
  </button>
</div>
