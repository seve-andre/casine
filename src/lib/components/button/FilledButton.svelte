<script lang="ts">
  export let disabled = false
  export let icon = null
</script>

<button on:click class="primary" class:disabled>
  <slot />
</button>

<style lang="scss">
  button {
    border: 0;
    cursor: pointer;
    border-radius: 20px;
    padding: 0 24px;
    height: 40px;
    font-weight: bold;
    box-shadow: none;
  }
  .primary {
    background: #6750a4;
    color: white;
  }
  .primary.disabled {
    pointer-events: none;
    background-color: whitesmoke;
    color: lightgrey;
  }
</style>
