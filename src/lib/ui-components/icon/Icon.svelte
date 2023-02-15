<script lang="ts">
  export let icon: string
  export let iconDescription = ""
</script>

<!-- empty alt 'cause it's a decorative icon -->
<img src={icon} alt={iconDescription} />
