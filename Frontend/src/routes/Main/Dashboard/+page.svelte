<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  

  let data = {};
  let interval; 
  let advancedView = false; 
  let numDecimals = 2; 

  onMount(async () => {
    await fetchStart();
    await fetchData();
    interval = setInterval(fetchData, 800);

    //return function runs when the component is unmounted 
    return() => {
      clearInterval(interval);
     
    };
  });

  async function fetchStart() {

      try {
        const response = await fetch("http://localhost:8000/start", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
  }
    catch(error){
      console.log("There was an error sending a post to /start endpoint.");
    }
  };
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


  function setAdvancedView(){
    advancedView = !advancedView;
    if(advancedView){
      numDecimals = 7; 
    }
    else{
      numDecimals = 2;  
    }
  }

  function toggleHelp(){
    
  }


</script>

<main class="container mx-auto">

  
   <div class="dropdown mr-3 mt-3">
  <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6a1.5 1.5 0 110 3 1.5 1.5 0 010-3zm0 4.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zm0 4.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3z" />
    </svg>
  </div>
  <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
  <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
      <li class="flex items-center">
      <label class="cursor-pointer flex items-center space-x-3 w-full hover:bg-gray-600">
        <input type="checkbox" class="checkbox checkbox-primary" on:click={setAdvancedView} />
        <span>Advanced view</span>
      </label>
    </li>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
   
    
   

       
  </ul>
</div>

  <!-- <button class="btn" onclick="my_modal_2.showModal()">Help</button>
    <dialog id="my_modal_2" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Dashboard Page</h3>
        <p class="py-4">The dashboard page contains metrics that you may use in order to help you decide 
          on when the best time to buy electricity is. 
        </p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog> -->


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
            <div class="stat-value">{value.toFixed(numDecimals)}V</div>
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
