<script lang="ts">
  import { Button } from "flowbite-svelte"
  import { goto } from "$app/navigation"
</script>

<!-- on:click={() => gotoApartmentInHouse(1, "A")} -->

<Button on:click={() => goto("/house/A/apartment/1")} pill>App. 1</Button>
