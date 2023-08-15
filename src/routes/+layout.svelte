<script lang="ts">
  import "../app.postcss"
  import logo from "$lib/assets/casine.png"
  import { LoadingPage, NavHeader, NavSidebar } from "$lib/ui-components"
  import { sidebarItems } from "$lib/ui-components/nav/sidebar/sidebar-items"

  import type { LayoutData } from "./$types"

  import { navigating, page } from "$app/stores"

  export let data: LayoutData

  $: activeUrl = $page.url.pathname
  $: shouldShowSidebar = sidebarItems.map(i => i.href).includes(activeUrl)

  const site = {
    name: data.appName,
    href: "/",
    img: logo
  }
</script>

<!-- https://kit.svelte.dev/docs/seo#manual-setup-title-and-meta -->
<svelte:head>
  <title>{$page.data.title}</title>
  <meta name="description" content={$page.data.description} />
</svelte:head>

{#if $navigating}
  <LoadingPage />
{:else}
  <div class="flex h-full">
    {#if shouldShowSidebar}
      <NavSidebar {activeUrl} brand={site} class="grow-0" />
    {/if}

    <div class="grow flex flex-col">
      {#if !shouldShowSidebar}
        <NavHeader class="flex-initial" />
      {/if}

      <!-- main fills up entire page height when the header is not shown -->
      <main class="flex-auto">
        <slot />
      </main>
    </div>
  </div>
{/if}
