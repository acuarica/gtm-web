<script>
  import Icon from "./Icon.svelte";
  import { faClock } from "@fortawesome/free-solid-svg-icons/faClock";
  import { hhmm } from "@gtm/format";

  export let files;

  let values;

  function lt(left, right) {
    return left.TimeSpent <= right.TimeSpent ? 1 : -1;
  }

  $: values = Object.values(files).sort(lt);

  let asc = true;
</script>

<button
  on:click={() => (asc = !asc)}
  type="button"
  class="block text-gray-400 hover:text-white focus:text-white
  focus:outline-none" />
Button
<ul>
  {#each values as file}
    <li class="small">
      {file.SourceFile} &nbsp;
      <Icon class="mb-1 h-4" icon={faClock} />
      {hhmm(file.TimeSpent)}
    </li>
  {/each}
</ul>
