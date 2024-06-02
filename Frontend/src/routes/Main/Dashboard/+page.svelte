<script>
  import { onMount } from "svelte";
  

  let data = [];
  let interval; 

  onMount(async () => {
    data = await fetchData();
    interval = setInterval(fetchData, 800);

    //return function runs when the component is unmounted 
    return() => {
      clearInterval(interval);
    };
  });

 

  async function fetchData() {
    //https://api.coindesk.com/v1/bpi/currentprice.json
    const response = await fetch("http://localhost:8001");//have to insert endpoint
    let text = await response.text()
    console.log(text)
    data = JSON.parse(text)
    console.log(data);
    return data;
  }
</script>

<main class="container mx-auto">
  <!-- <h1 class="text-2xl font-bold mb-4">Overview</h1>
  <p>Here you can overview data about the grid.</p> -->
  

{#if data}
    {#each Object.entries(data) as [key, value]}
      
        
        {#if key === "Price"}
          <div class="stats shadow">
            <div class="stat place-items-center">
              <div class="stat-title">Current Price</div>
              <div class="stat-value">R{value}</div>
              <div class="stat-desc">Prices are dynamic</div>
          </div>
        </div>
        {:else}
        <div class = "stats shadow">
          <div class="stat place-items-center">
            <div class="stat-title">Current Voltage</div>
            <div class="stat-value">{value}V</div>
            <div class="stat-desc text-red-700">90 (14%)</div>
          </div>
          </div>
        {/if}
      
    {/each}
  {:else}
    <p>Loading...</p>
  {/if}


 

   
<!--    -->
<!--    <div class="stat place-items-center">-->
<!--      <div class="stat-title">Users</div>-->
<!--      <div class="stat-value">4,200</div>-->
<!--      <div class="stat-desc text-green-800">↗︎ 40 (2%)</div>-->
<!--    </div>-->
    
    
    
<!--  </div>-->





</main>

<style>
  
</style>
