<script lang="ts">
  import "../app.css";

  interface Props {
    children?: import("svelte").Snippet;
  }

  let { children }: Props = $props();
</script>

<svelte:window oncontextmenu={(e) => e.preventDefault()} />
<div class="text-sm text-white">
  {@render children?.()}
</div>
