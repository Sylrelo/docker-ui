<script lang="ts">
  import Containers from "./lib/Containers.svelte";

  import "@tabler/icons-webfont/tabler-icons.min.css";
  import Menu from "./Menu.svelte";

  // import { onMount } from "svelte";

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   const greetMsg = await invoke("socket_communication", {
  //     method: "GET",
  //     endpoint: "http://localhost/version",
  //   });

  //   console.log(greetMsg);
  // }

  // onMount(() => {
  //   greet();
  // });
</script>

<Menu />
<main class="container">
  <!-- <button type="button" on:click={() => greet()}> Button </button> -->
  <Containers />
</main>

<style>
  .container {
    margin-left: 74px;
    width: calc(100% - 84px);
  }
</style>
