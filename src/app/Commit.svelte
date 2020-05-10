<script>
  import { hhmm } from "@gtm/notes";
  import Icon from "./Icon.svelte";
  import { faClock } from "@fortawesome/free-solid-svg-icons/faClock";
  import { faCog } from "@fortawesome/free-solid-svg-icons/faCog";
  import { faFileCode } from "@fortawesome/free-solid-svg-icons/faFileCode";
  import FileNotes from "./FileNotes.svelte";

  export let commit;

  let toggleVisible;
</script>

<div class="shadow-md p-2">
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
  <span>
    <span class="mb-1">{commit.Subject}</span>
    <small class="mb-1">
      {#each commit.Message ? commit.Message.split('\n') : [] as line}
        <br />
        <span>{line}</span>
      {/each}
    </small>
  </span>

  <button
    class="m-1 p-1"
    on:click={() => (toggleVisible = !toggleVisible)}
    type="button">
    <Icon class="mb-1 h-4" icon={faFileCode} />
    ...
  </button>
  <div>
    {#if toggleVisible}
      <FileNotes files={commit.Note.Files} />
    {/if}
  </div>
</div>
