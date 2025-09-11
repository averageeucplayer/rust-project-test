<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import "../app.css";
  import { onMount } from "svelte";
  import Loader from "./loader.svelte";
  import { data } from "$lib/store";

  interface Props {
    children?: import("svelte").Snippet;
  }

  interface LoadResult {}

  type AppEvent = {

  };

  let { children }: Props = $props();
  let isLoading = $state(true);

  onMount(async () => {
    console.log("frontend loading");
    const loadResult = await invoke<LoadResult>("load");
    console.log("frontend loaded");
    isLoading = false;

    listen<AppEvent>("app-event", (event) => {
      console.log(event.payload);
      data.set(event.payload)
    })


  });
</script>

  {#if isLoading}
    <Loader />
  {:else}
    <div class="text-sm text-white">
      {@render children?.()}
    </div>
  {/if}
