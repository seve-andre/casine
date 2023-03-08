<script lang="ts">

  export let placeholder: string
  export let text: string

  let onSearch: (text: string) => void

  const resetText = () => (text = "")
</script>

<div class="search-input">
  <!-- <TextInput bind:value={text} aria-label={placeholder} {placeholder} icon={MagnifyingGlass} variant="filled">
    <svelte:fragment slot="rightSection">
      {#if text.length > 0}
        <ActionIcon title="Cancella" on:click={resetText}>
          <Cross2 />
        </ActionIcon>
      {/if}
    </svelte:fragment>
  </TextInput>
  <ActionIcon color="blue" variant="filled" title={placeholder} size={36}>
    <MagnifyingGlass size={20} />
  </ActionIcon> -->
</div>

<style>
  .search-input {
    display: flex;
    align-items: center;
    gap: 1%;
  }
</style>
