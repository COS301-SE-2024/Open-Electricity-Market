<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  

  let data = {};
  let interval; 

  onMount(async () => {
    await fetchData();
    interval = setInterval(fetchData, 800);

    //return function runs when the component is unmounted 
    return() => {
      clearInterval(interval);
     
    };
  });

 

 async function fetchData() {

      try {
        const response = await fetch("http://localhost:8000/overview", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
        console.log("request being sent...");
        console.log(response);
        // const response = fetch("http://localhost:8000");
        const fdata = await response.json();
        console.log(data);
        //Voltage 1,2,3 as well as price
        data = fdata; 
      } catch (error) {
        console.log("There was an error fetching the JSON for the overview..", error);
      }
  };
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
            <div class="stat-title">{key} Voltage</div>
            <div class="stat-value">{value.toFixed(2)}V</div>
            <div class="stat-desc text-red-700">90 (14%)</div>
          </div>
          </div>
        {/if}
      
    {/each}
  {:else}
    <p>Loading...</p>
  {/if}

  <Chart class = "w-3/5 h-3/5" {data} />


 

   
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
