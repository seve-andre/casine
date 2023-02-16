<script lang="ts">
  export let disabled = false
  // export let icon = null
</script>

<button on:click class="outlined-btn" class:disabled>
  <slot />
</button>

<style>
  .outlined-btn {
    background-color: transparent;
    border: 1px solid #79747e;
    color: #6750a4;
  }

  .outlined-btn.disabled {
    pointer-events: none;

    /* 1f = 12% opacity */
    /* background-color: #1c1b1f1f; */

    /* 61 = 38% opacity */
    /* color: #1c1b1f61; */
  }

  .outlined-btn:hover {
    /* box-shadow: 1px 0px 1px black; */
  }

  .outlined-btn:focus {
  }
</style>
