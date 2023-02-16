<script lang="ts">
  export let disabled = false
  // export let icon = null
</script>

<button on:click class="text-btn" class:disabled>
  <slot />
</button>

<style>
  .text-btn {
    background-color: transparent;
    color: #6750a4;
  }

  .text-btn.disabled {
    pointer-events: none;

    /* 1f = 12% opacity */
    /* background-color: #1c1b1f1f; */

    /* 61 = 38% opacity */
    /* color: #1c1b1f61; */
  }

  .text-btn:hover {
    /* box-shadow: 1px 0px 1px black; */
  }

  .text-btn:focus {
  }
</style>
