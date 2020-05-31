<script>
  import Icon from "./Icon.svelte";
  import { faClock } from "@fortawesome/free-solid-svg-icons/faClock";
  import { hhmm } from "@gtm/notes";

  export let files;

  let values;

  function lt(left, right) {
    return left.TimeSpent <= right.TimeSpent ? 1 : -1;
  }

  $: values = Object.values(files).sort(lt);
</script>

<button
  type="button"
  class="block text-gray-400 hover:text-white focus:text-white
  focus:outline-none">
</button>
<ul>
  {#each values as file}
    <li class="small truncate">
      <Icon class="mb-1 h-3" icon={faClock} />
      {hhmm(file.TimeSpent)} {file.SourceFile} &nbsp;
    </li>
  {/each}
</ul>
