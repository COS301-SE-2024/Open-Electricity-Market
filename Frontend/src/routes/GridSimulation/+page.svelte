<script>
  import { onMount } from "svelte";
  

  let data = [];
  let interval; 

  onMount(async () => {
    data = await fetchData();
    // interval = setInterval(fetchData, 2000);

    //return function runs when the component is unmounted 
    return() => {
      clearInterval(interval);
    };
  });

 

  async function fetchData() {
    const response = await fetch("https://api.coindesk.com/v1/bpi/currentprice.json"); //have to insert endpoint
    data = await response.json();
    console.log(data);
    return data;
  }
</script>

<main class="container mx-auto">
  <h1 class="text-2xl font-bold mb-4">Grid Simulation</h1>
  <p>Here you can simulate the national grid.</p>

  {#if data}
    {#each Object.entries(data) as [key, value]}
      <p>
        {key}:
        {#if key === "price"}
          ${value}
        {:else}
          {Array.isArray(value) ? value.join(", ") : value}
        {/if}
      </p>
    {/each}
  {:else}
    <p>Loading...</p>
  {/if}
</main>

<style>
  
</style>
