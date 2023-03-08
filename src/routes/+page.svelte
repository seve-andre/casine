<script lang="ts">
  import { Button, Tooltip } from "flowbite-svelte"
  import { goto } from "$app/navigation"
  import FilledButton from "@/lib/ui-components/button/FilledButton.svelte"
</script>

<!-- on:click={() => gotoApartmentInHouse(1, "A")} -->

<!-- <Button on:click={() => goto("/house/A/apartment/1")} pill>App. 1</Button> -->

<FilledButton on:click={() => goto("/house/A/apartment/1")}>App. 1</FilledButton>
<Tooltip placement="bottom">Vai all'appartmento 1</Tooltip>
