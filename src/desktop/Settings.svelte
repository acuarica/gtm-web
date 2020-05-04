<script>
  const versions = {
    node: "Node.js",
    chrome: "Chromium",
    electron: "Electron"
  };
</script>

<h4 class="text-white">About</h4>
{#each ['node', 'chrome', 'electron'] as engine}
  <div>
    {versions[engine]}
    <span id="node-version">{process.versions[engine]}</span>
  </div>
{/each}
