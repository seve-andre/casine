<script>
  import "../app.postcss"
  import { Button, Navbar } from "flowbite-svelte"

  import { page } from "$app/stores"
  import { goBack } from "./navigation-utils"
  import LeftArrow from "@/lib/ui-components/icon/LeftArrow.svelte"
</script>

<div class="flex flex-col h-screen">
  {#if $page.route.id != "/"}
    <header class="flex-initial">
      <Navbar class="bg-gray-50 dark:bg-gray-900">
        <Button pill on:click={() => goBack()} color="light">
          <LeftArrow clazz="mr-2 -ml-1 w-5 h-5" />
          Torna indietro
        </Button>
      </Navbar>
    </header>
  {/if}
  <main class="px-10 py-5 h-screen flex-auto overflow-hidden bg-gray-50 dark:bg-gray-900">
    <slot />
  </main>
</div>
