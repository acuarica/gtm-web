<script>
  import { hhmm } from "@gtm/notes";
  import Icon from "./Icon.svelte";
  import { faCalendar } from "@fortawesome/free-solid-svg-icons/faCalendar";
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
      <span class="font-bold">{commit.Project}</span>
      &nbsp;
      <span
        class="text-sm px-3 py-1 bg-gray-600 rounded-full">
        {commit.Author}
      </span>
    </div>
    <div>
      <small class="mb-2">
        <Icon class="mb-1 h-4" icon={faClock} />
        {hhmm(commit.timeSpent)}
      </small>
      &nbsp;
      <small class="text-muted">
        <Icon class="mb-1 h-4" icon={faCalendar} />
        {commit.When}
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
      {#if commit.Note && commit.Note.Files}
        <FileNotes files={commit.Note.Files} />
      {:else}
        <div class="text-red-400 italic">No files in commit</div>
      {/if}
    {/if}
  </div>
</div>
