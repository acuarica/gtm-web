<script>
  import { hhmm } from "@gtm/format";

  export let files;
</script>

<ul>
  {#each files as file}
    <li class="small">
      {file.SourceFile} &nbsp;
      <i class="fas fa-clock" />
      {hhmm(file.TimeSpent)}
    </li>
  {/each}
</ul>
