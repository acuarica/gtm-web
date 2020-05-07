<script>
  import Box from "./Box.svelte";

  export let title;
  export let body;
  export let footer;
</script>

<Box class="p-5 flex flex-col justify-around {$$props.class}">
  <div class="text-xl">{title}</div>
  <div class="text-2xl text-highlight font-bold">{body}</div>
  <div class="text-sm text-muted">{footer}</div>
</Box>
