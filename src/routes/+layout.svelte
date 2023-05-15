<script lang="ts">
  import "~/app.postcss"
  import { navigating, page } from "$app/stores"
  import {
    Spinner,
    Sidebar,
    SidebarWrapper,
    SidebarGroup,
    SidebarItem,
    Navbar,
    Button,
    SidebarBrand,
    Tooltip,
  } from "flowbite-svelte"
  import {
    BookFilled,
    BookOutlined,
    HomeFilled,
    HomeOutlined,
    LeftArrow,
    PriceTagFilled,
    PriceTagOutlined,
  } from "~/lib/ui-components"
  import logo from "~/lib/assets/casine.png"
  import type { LayoutData } from "./$types"

  export let data: LayoutData

  $: activeUrl = $page.url.pathname

  const sidebarItems = [
    {
      name: "Principale",
      href: "/",
      outlinedIcon: HomeOutlined,
      filledIcon: HomeFilled,
    },
    {
      name: "Prenotazioni",
      href: "/bookings",
      outlinedIcon: BookOutlined,
      filledIcon: BookFilled,
    },
    {
      name: "Prezzi",
      href: "/prices",
      outlinedIcon: PriceTagOutlined,
      filledIcon: PriceTagFilled,
    },
  ]
  $: shouldShowSidebar = sidebarItems.map(i => i.href).includes(activeUrl) || activeUrl === ""

  const site = {
    name: data.appName,
    href: "/",
    img: logo,
  }
</script>

<!-- https://kit.svelte.dev/docs/seo#manual-setup-title-and-meta -->
<svelte:head>
  <title>{$page.data.title}</title>
  <meta name="description" content={$page.data.description} />
</svelte:head>

{#if $navigating}
  <div class="spinner-wrapper">
    <Spinner />
  </div>
{:else}
  <div class="page">
    {#if shouldShowSidebar}
      <div class="page__sidebar">
        <Sidebar>
          <SidebarWrapper>
            <SidebarGroup>
              <SidebarBrand {site} />
              {#each sidebarItems as item}
                {@const isActive = activeUrl === item.href}
                <SidebarItem
                  label={item.name}
                  href={item.href}
                  active={isActive}
                  spanClass="{isActive ? 'font-bold' : 'font-normal'} ml-3"
                >
                  <svelte:fragment slot="icon">
                    <svelte:component this={isActive ? item.filledIcon : item.outlinedIcon} class="w-6 h-6" />
                  </svelte:fragment>
                </SidebarItem>
              {/each}
            </SidebarGroup>
          </SidebarWrapper>
        </Sidebar>
      </div>
    {/if}

    <div class="page__content">
      {#if !shouldShowSidebar}
        <header class="page__content__header">
          <Navbar>
            <Button href="/" pill outline>
              <LeftArrow class="w-4 h-4" />
            </Button>
          </Navbar>
        </header>
      {/if}

      <main class="page__content__main">
        <slot />
      </main>
    </div>
  </div>
{/if}

<style>
  .spinner-wrapper {
    /* make spinner fill entire page */
    height: 100%;

    /* show spinner at the center of the screen*/
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .page {
    display: flex;
    height: 100%;
  }

  .page__sidebar {
    flex: 0 1 20%;

    /* match Flowbite Sidebar bg color */
    background-color: rgb(249, 250, 251);
  }

  .page__content {
    flex: 1 1 80%;
    display: flex;
    flex-direction: column;
  }

  .page__content__header {
    flex: 0 1 auto;
  }

  .page__content__main {
    /* main fills up entire page height when the header is not shown */
    flex: 1 1 auto;
  }
</style>
