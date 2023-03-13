<script>
  import { Button, DarkMode, Navbar } from "flowbite-svelte"
  import "../app.postcss"

  import { page } from "$app/stores"
  import { goBack } from "./navigation"
</script>

<div class="flex flex-col h-screen">
  {#if $page.route.id != "/"}
    <header class="flex-initial">
      <Navbar class="bg-gray-50 dark:bg-gray-900">
        <Button pill on:click={() => goBack()} color="light">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true" class="mr-2 -ml-1 w-5 h-5">
            <g data-name="Layer 2">
              <g data-name="arrow-back">
                <rect width="24" height="24" transform="rotate(90 12 12)" opacity="0" />
                <path
                  d="M19 11H7.14l3.63-4.36a1 1 0 1 0-1.54-1.28l-5 6a1.19 1.19 0 0 0-.09.15c0 .05 0 .08-.07.13A1 1 0 0 0 4 12a1 1 0 0 0 .07.36c0 .05 0 .08.07.13a1.19 1.19 0 0 0 .09.15l5 6A1 1 0 0 0 10 19a1 1 0 0 0 .64-.23 1 1 0 0 0 .13-1.41L7.14 13H19a1 1 0 0 0 0-2z"
                />
              </g>
            </g>
          </svg>
          Torna indietro
        </Button>
      </Navbar>
    </header>
  {/if}
  <main class="px-10 py-5 h-screen flex-auto overflow-hidden bg-gray-50 dark:bg-gray-900">
    <slot />
  </main>
</div>
