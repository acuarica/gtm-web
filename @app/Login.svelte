<script>
  import logo from "./assets/gtm-logo.png";
  import githubLogo from "./assets/GitHub-Mark-Light-64px.png";
  import { faAngleRight } from "@fortawesome/free-solid-svg-icons/faAngleRight";
  import Icon from "./Icon.svelte";
</script>

<div class="flex justify-center items-center">
  <a href="./">
    <img class="inline h-8" src={logo} alt="gtm Logo" />
    <span class="text-white">Dashboard</span>
  </a>
  <span class="ml-2 font-medium text-white">
    <Icon class="mr-2 mb-1 h-4" icon={faAngleRight} />
    Login
  </span>
  <a
    href="https://github.com/login/oauth/authorize?client_id=c0d72815c0f4d616112a">
    <img src={githubLogo} alt="GitHub mark logo" />
    Login with github
  </a>
</div>
