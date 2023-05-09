<script lang="ts">
  import "~/app.postcss"
  import { navigating, page } from "$app/stores"
  import { Spinner, Sidebar, SidebarWrapper, SidebarGroup, SidebarItem, Navbar, Button } from "flowbite-svelte"
  import {
    BookFilled,
    BookOutlined,
    HomeFilled,
    HomeOutlined,
    LeftArrow,
    PriceTagFilled,
    PriceTagOutlined,
  } from "~/lib/ui-components"

  $: activeUrl = $page.url.pathname
  $: shouldShowSidebar = sidebarItems.map(i => i.href).includes(activeUrl)

  const sidebarItems = [
    {
      name: "Home",
      href: "/",
      outlinedIcon: HomeOutlined,
      filledIcon: HomeFilled,
    },
    {
      name: "Bookings",
      href: "/bookings",
      outlinedIcon: BookOutlined,
      filledIcon: BookFilled,
    },
    {
      name: "Prices",
      href: "/prices",
      outlinedIcon: PriceTagOutlined,
      filledIcon: PriceTagFilled,
    },
  ]
</script>

<!-- https://kit.svelte.dev/docs/seo#manual-setup-title-and-meta -->
<svelte:head>
  <title>{$page.data.title}</title>
  <meta name="description" content={$page.data.description} />
</svelte:head>

{#if $navigating}
  <div class="h-screen flex justify-center items-center">
    <Spinner />
  </div>
{:else}
  <div class="layout-container h-full">
    {#if shouldShowSidebar}
      <div class="sidebar bg-gray-50">
        <Sidebar>
          <SidebarWrapper>
            <SidebarGroup>
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

    <div class="content-container">
      {#if !shouldShowSidebar}
        <header>
          <Navbar>
            <Button href="/" pill color="light" outline>
              <LeftArrow class="w-4 h-4" />
            </Button>
          </Navbar>
        </header>
      {/if}

      <main>
        <slot />
      </main>
    </div>
  </div>
{/if}

<style>
  .layout-container {
    display: flex;
  }

  .sidebar {
    flex-basis: 20%;
  }

  .content-container {
    flex-basis: 80%;
    flex-grow: 1;
  }
</style>
