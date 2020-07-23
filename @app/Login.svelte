<script>
  import logo from "./assets/gtm-logo.png";
  import githubLogo from "./assets/GitHub-Mark-Light-64px.png";
  import Icon from "./Icon.svelte";
  import Box from "./Box.svelte";
</script>

<div class="py-24">
  <Box class="flex mx-auto max-w-sm items-center justify-center">

    <div class="p-8">
      <h2 class="text-2xl font-semibold text-gray-700 text-center">
        <img class="inline h-8" src={logo} alt="gtm Logo" />
        <span class="text-white">Dashboard</span>
      </h2>
      <p class="text-xl text-gray-200 text-center">Welcome!</p>
      <a
        class="flex items-center justify-center mt-4 rounded-lg shadow-xl
        bg-gray-600 hover:bg-gray-500"
        href="https://github.com/login/oauth/authorize?client_id=c0d72815c0f4d616112a">
        <div class="px-4 py-3">
          <img src={githubLogo} alt="GitHub mark logo" />
        </div>
        <h1 class="px-4 py-3 w-5/6 text-center text-gray-100 font-bold">
          Sign in with GitHub
        </h1>
      </a>
    </div>
  </Box>

</div>
