<script lang="ts">
  import { Table } from "flowbite-svelte"
</script>

<Table striped divClass="table__main">
  <slot />
</Table>

<style>
  :global(.table__main) {
    height: 100%;

    border-width: 1px;
    border-style: solid;
    /* dark: rgba(255, 255, 255, 0.12) */
    border-color: rgba(0, 0, 0, 0.12);
    border-radius: 5px;

    /* so that table corners don't get overlapped */
    padding: 0.25rem;
  }
</style>
