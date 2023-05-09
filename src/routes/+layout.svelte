<script lang="ts">
  import "~/app.postcss"
  import { navigating, page } from "$app/stores"
  import { Spinner, Sidebar, SidebarWrapper, SidebarGroup, SidebarItem, Navbar, Button } from "flowbite-svelte"
  import { HomeFilled, HomeOutlined, LeftArrow } from "~/lib/ui-components"

  $: activeUrl = $page.url.pathname

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
      outlinedIcon: LeftArrow,
      filledIcon: HomeFilled,
    },
    {
      name: "Prices",
      href: "/prices",
      outlinedIcon: LeftArrow,
      filledIcon: HomeFilled,
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
  <div class="layout-container h-screen">
    <div class="sidebar bg-gray-50">
      <Sidebar>
        <SidebarWrapper>
          <SidebarGroup>
            {#each sidebarItems as item}
              {@const isActive = activeUrl === item.href}
              <SidebarItem label={item.name} href={item.href} active={isActive}>
                <svelte:fragment slot="icon">
                  <svelte:component this={isActive ? item.filledIcon : item.outlinedIcon} class="w-6 h-6" />
                </svelte:fragment>
              </SidebarItem>
            {/each}
          </SidebarGroup>
        </SidebarWrapper>
      </Sidebar>
    </div>

    <div class="content-container">
      {#if ["/", "/bookings", "/prices"].includes(activeUrl) === false}
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
  }
</style>
