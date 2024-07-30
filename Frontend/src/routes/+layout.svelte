<script>
  import "../app.css";
  import { page } from '$app/stores';
  import { derived } from "svelte/store";

  let activebutton = '';

  const currentpath = derived(page, $page => $page.url.pathname);

  $: activebutton = $currentpath;

  import { fade } from "svelte/transition";
  import { cubicIn, cubicOut } from "svelte/easing";
  import { fly } from "svelte/transition";
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
