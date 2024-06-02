<script>
  import "../app.css";
  import { page } from '$app/stores';
  import { derived } from "svelte/store";

  let activebutton = '';

  const currentpath = derived(page, $page => $page.url.pathname);

  $: activebutton = $currentpath; 


  

  


</script>

<style>
 

</style>



<main class="container mx-auto ">
  <slot />
</main>