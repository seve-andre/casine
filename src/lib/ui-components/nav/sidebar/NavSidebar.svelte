<script lang="ts">
  import { Sidebar, SidebarWrapper, SidebarGroup, SidebarBrand, SidebarItem } from "flowbite-svelte"
  import type { SiteType } from "flowbite-svelte/dist/types"

  import { sidebarItems } from "./sidebar-items"

  export let activeUrl: string
  export let brand: SiteType
</script>

<Sidebar class={$$props.class}>
  <SidebarWrapper class="h-full">
    <SidebarGroup>
      <SidebarBrand site={brand} />
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
