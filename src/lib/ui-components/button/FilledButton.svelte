<!-- https://m3.material.io/components/buttons/specs#0b1b7bd2-3de8-431a-afa1-d692e2e18b0d -->
<script lang="ts">
  export let disabled = false
  // export let icon = null
</script>

<button on:click class="primary-btn" class:disabled>
  <slot />
</button>

<style>
  .primary-btn {
    background-color: #6750a4;
    color: #fff;
  }

  .primary-btn.disabled {
    pointer-events: none;

    /* 1f = 12% opacity */
    background-color: #1c1b1f1f;

    /* 61 = 38% opacity */
    color: #1c1b1f61;
  }

  .primary-btn:hover {
    box-shadow: 1px 0px 1px black;
  }

  .primary-btn:focus {
  }
</style>
