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
  <h1 class="text-2xl font-bold mb-4">Overview</h1>
  <p>Here you can overview data about the grid.</p>
  

  <!-- {#if data}
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
  {/if} -->


  <div class="stats shadow">
  
    <div class="stat place-items-center">
      <div class="stat-title">Current Price</div>
      <div class="stat-value">R31</div>
      <div class="stat-desc">Prices are dynamic</div>
    </div>

    <div class="stat place-items-center">
      <div class="stat-title">Current Voltage</div>
      <div class="stat-value">5V</div>
      <div class="stat-desc text-red-700">90 (14%)</div>
    </div>
    
    <div class="stat place-items-center">
      <div class="stat-title">Users</div>
      <div class="stat-value">4,200</div>
      <div class="stat-desc text-green-800">↗︎ 40 (2%)</div>
    </div>
    
    
    
  </div>





</main>

<style>
  
</style>
