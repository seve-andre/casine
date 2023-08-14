<script lang="ts">
  import { Table } from "flowbite-svelte"
</script>

<!-- p-1 so that table corners don't get overlapped -->
<Table
  striped
  divClass="h-full border border-solid rounded-md border-black/[0.12] dark:border-white/[0.12] p-1"
>
  <slot />
</Table>
