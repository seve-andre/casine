<script lang="ts">
  import "../app.css"
  import {
    Sidebar,
    SidebarWrapper,
    SidebarGroup,
    SidebarItem,
    Navbar,
    SidebarBrand
  } from "flowbite-svelte"

  import logo from "~/lib/assets/casine.png"
  import {
    BookFilled,
    BookOutlined,
    HomeFilled,
    HomeOutlined,
    IconButton,
    LeftArrow,
    LoadingPage,
    PriceTagFilled,
    PriceTagOutlined
  } from "~/lib/ui-components"

  import type { LayoutData } from "./$types"

  import { navigating, page } from "$app/stores"

  export let data: LayoutData

  $: activeUrl = $page.url.pathname

  const sidebarItems = [
    {
      name: "Principale",
      href: "/",
      outlinedIcon: HomeOutlined,
      filledIcon: HomeFilled
    },
    {
      name: "Prenotazioni",
      href: "/bookings",
      outlinedIcon: BookOutlined,
      filledIcon: BookFilled
    },
    {
      name: "Prezzi",
      href: "/prices",
      outlinedIcon: PriceTagOutlined,
      filledIcon: PriceTagFilled
    }
  ]
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
      <Sidebar class="grow-0">
        <SidebarWrapper class="h-full">
          <SidebarGroup>
            <SidebarBrand {site} />
            {#each sidebarItems as item}
              {@const isActive = activeUrl === item.href}
              <SidebarItem
                label={item.name}
                href={item.href}
                active={isActive}
                spanClass={isActive ? "font-bold" : "font-normal"}
                class="flex gap-2"
              >
                <!-- icon color in "class" below -->
                <svelte:fragment slot="icon">
                  <svelte:component
                    this={isActive ? item.filledIcon : item.outlinedIcon}
                    class="w-6 h-6"
                  />
                </svelte:fragment>
              </SidebarItem>
            {/each}
          </SidebarGroup>
        </SidebarWrapper>
      </Sidebar>
    {/if}

    <div class="grow flex flex-col">
      {#if !shouldShowSidebar}
        <header class="flex-initial">
          <Navbar navDivClass="mx-1 flex flex-wrap justify-between items-center">
            <IconButton href="/">
              <LeftArrow class="w-4 h-4" />
            </IconButton>
          </Navbar>
        </header>
      {/if}

      <!-- main fills up entire page height when the header is not shown -->
      <main class="flex-auto">
        <slot />
      </main>
    </div>
  </div>
{/if}
