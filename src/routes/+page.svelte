<script lang="ts">
  import { Heading, Tooltip } from "flowbite-svelte"

  import { FilledButton } from "~/lib/ui-components"

  import type { PageData } from "./$types"

  export let data: PageData
</script>

<div class="homepage">
  <div class="houses">
    {#each data.housesWithApartments as { house, apartments }}
      <div class="house">
        <Heading customSize="house__title title-text">Casa {house.house_name}</Heading>
        <div class="house__choices">
          {#each apartments as apartment}
            <FilledButton href={`apartments/${apartment.id}`}>
              App. {apartment.apartment_number}
            </FilledButton>
            <Tooltip placement="bottom">Vai all'appartamento {apartment.apartment_number}</Tooltip>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>

<style>
  .homepage {
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.25rem 2.5rem;
  }

  .houses {
    display: flex;
    flex-direction: column;
    gap: 3rem;
  }

  .house {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  :global(.house__title) {
    font-size: 1.5rem;
    line-height: 2rem;
  }

  .house__choices {
    /* show 3 buttons per line */
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;

    /* don't fill entire width, so buttons don't look "weird" */
    width: 70%;
  }
</style>
