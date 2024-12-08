<script lang="ts">
    import { getRecords } from "../api";
    import { type AppRecord } from "web-core";
    import { onMount } from "svelte";
    import { Button } from "web-component";

    console.log(Button);

    let items = $state<AppRecord[]>([]);

  onMount(() => {
    (async () => {
         items = await getRecords();
    })();

  })

</script>

<div class="container">
    <Button>
      Test
    </Button>
    <ul>
    {#each items as item}
        <li>{item.name}</li>
    {/each}
    </ul>


</div>
