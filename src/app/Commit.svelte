<script>
  import { hhmm } from "@gtm/format";
  import Icon from "./Icon.svelte";
  import { faClock } from "@fortawesome/free-solid-svg-icons/faClock";
  import { faCog } from "@fortawesome/free-solid-svg-icons/faCog";
  import FileNotes from "./FileNotes.svelte";

  export let commit;

  let toggleVisible;
</script>

<div class="shadow-md">
  <div class="flex justify-between">
    <div class="mb-2">
      <span class="badge badge-pill badge-primary">{commit.Project}</span>
      <span class="badge badge-pill abadge-light text-muted">
        {commit.Author}
      </span>
    </div>
    <div>
      <small class="mb-2">
        <Icon class="mb-1 h-4" icon={faClock} />
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
    <Icon class="mb-1 h-4" icon={faCog} />
  </button>
  <div>
    {#if toggleVisible}
      <FileNotes files={commit.Note.Files} />
    {/if}
  </div>
</div>
