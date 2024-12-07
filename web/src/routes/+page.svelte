<script lang="ts">
    import { type Record, getRecords } from "web-lib";
    import { onMount } from "svelte";
    import { Button } from "web-component";
    let items = $state<Record[]>([]);

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
        <li>{item}</li>
    {/each}
    </ul>

  
</div>
