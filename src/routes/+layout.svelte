<script lang="ts">
  import "~/app.css"
  import { navigating, page } from "$app/stores"
  import { Spinner, Sidebar, SidebarWrapper, SidebarGroup, SidebarItem, Navbar, SidebarBrand } from "flowbite-svelte"
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
  import IconButton from "~/lib/ui-components/button/IconButton.svelte"

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
        <Sidebar asideClass="w-auto">
          <SidebarWrapper>
            <SidebarGroup ulClass="sidebar__group">
              <SidebarBrand {site} />
              {#each sidebarItems as item}
                <!-- home sidebar active url is "" -->
                {@const isActive = activeUrl === item.href}
                <SidebarItem
                  label={item.name}
                  href={item.href}
                  active={isActive}
                  spanClass={isActive ? "font-bold" : "font-normal"}
                  aClass="sidebar__item"
                  activeClass="sidebar__item"
                >
                  <!-- icon color in "class" below -->
                  <svelte:fragment slot="icon">
                    <div class:sidebar__item__icon-active={isActive}>
                      <svelte:component this={isActive ? item.filledIcon : item.outlinedIcon} class="w-6 h-6" />
                    </div>
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
          <Navbar navDivClass="mx-1 flex flex-wrap justify-between items-center">
            <IconButton href="/">
              <LeftArrow class="w-4 h-4" />
            </IconButton>
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
    flex: 0 10rem;

    /* match Flowbite Sidebar bg color */
    background-color: rgb(249, 250, 251);
  }

  .page__content {
    flex: 1;
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

  :global(.sidebar__group) {
    display: flex;
    flex-direction: column;

    /* gap between each item in the sidebar */
    gap: 0.75rem;
  }

  :global(.sidebar__item) {
    display: flex;
    align-items: center;
    flex-direction: column;

    /* gap between icon and label */
    gap: 0.25rem;
  }

  :global(.sidebar__item__icon-active) {
    background-color: var(--md-sys-color-secondary-container);
    border-radius: 1rem;
    height: 2rem;
    width: 3.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
