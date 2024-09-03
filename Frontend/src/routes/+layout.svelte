<script>
  import "../app.css";

  import { fade } from "svelte/transition";
  import { cubicIn, cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { onMount } from "svelte";
  export let data;
</script>



{#key data.pathname}
    <div
    in:fly={{ easing: cubicOut, y: 10, duration: 300, delay: 500 }}
    out:fly={{ easing: cubicIn, y: -10, duration: 300 }}
    >
		<slot />
	</div>
{/key} 
  