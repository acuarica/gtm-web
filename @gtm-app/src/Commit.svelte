<script>
  import { hhmm } from "@gtm/notes";

  export let commit;

  let toggleVisible;
</script>

<div class="group">
  <div class="flex justify-between">
    <div class="mb-2">
      <span class="badge badge-pill badge-primary">{commit.Project}</span>
      <span class="badge badge-pill badge-light text-muted">
        {commit.Author}
      </span>
    </div>
    <div>
      <small class="mb-2">
        <i class="fas fa-clock" />
        {hhmm(commit.timeSpent)}
      </small>
      <small class="text-muted">
        <i class="fa fa-calendar" />
        &nbsp; {commit.When}
      </small>
    </div>
  </div>
  <h6 class="mb-1">{commit.Subject}</h6>
  <small class="mb-1">{commit.Message.replace('\n', '<br>')}</small>

  <button
    class="btn btn-outline-secondary btn-sm"
    on:click={() => (toggleVisible = !toggleVisible)}
    type="button">
    <i class="fas fa-cog" />
  </button>
  <div>
    {#if toggleVisible}
      <ul>
        {#each commit.Note.Files as file}
          <li class="small">
            {file.SourceFile} &nbsp;
            <i class="fas fa-clock" />
            {hhmm(file.TimeSpent)}
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>
