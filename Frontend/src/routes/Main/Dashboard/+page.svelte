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
        const data = await response.json();
        console.log(data);
        //Voltage 1,2,3 as well as price
        return data; 
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
