<script>
  import "~/app.postcss"
  import { Button, Navbar } from "flowbite-svelte"
  import { page } from "$app/stores"
  import LeftArrow from "~/lib/ui-components/icon/LeftArrow.svelte"
</script>

<div class="flex flex-col h-screen">
  {#if $page.url.pathname != "/"}
    <header class="flex-initial">
      <Navbar class="bg-gray-50 dark:bg-gray-900">
        <Button href="/" pill color="light" outline>
          <LeftArrow clazz="w-4 h-4" />
        </Button>
      </Navbar>
    </header>
  {/if}
  <main class="px-10 py-5 h-screen flex-auto overflow-hidden bg-gray-50 dark:bg-gray-900">
    <slot />
  </main>
</div>
