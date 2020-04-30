<script>
  export let title;
  export let body;
  export let footer;
</script>

<div
  class="rounded bg-white shadow-md h-32 w-48 p-5 flex flex-col justify-around">
  <div class="text-base text-gray-600">{title}</div>
  <div class="text-2xl text-gray-700 font-bold">{body}</div>
  <div class="text-sm text-gray-600">{footer}</div>
</div>
