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

  
   <!-- <div class="form-control top-right">
  <label class="label cursor-pointer">
    <span class="label-text mr-2">Advanced view</span>
    <input type="checkbox" class="toggle" checked={advancedView} on:change={setAdvancedView} />
  </label>
  </div> -->

  <div class="top-right">
    <div class="stats stats-vertical shadow">
      <div class="stat">
        <div class="stat-title">Available Balance</div>
        <div class="stat-value">31K</div>
        <div class="stat-desc">Jul 1st - Aug 1st</div>
      </div>
    
      <div class="stat">
        <div class="stat-title"></div>
        <div class="stat-value">4,200</div>
       
      </div>
    
      <div class="stat">
        <div class="stat-title">New Registers</div>
        <div class="stat-value">1,200</div>
       
      </div>
    </div>
  </div>

  <button class="btn" onclick="my_modal_2.showModal()">Help</button>
    <dialog id="my_modal_2" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Dashboard Page</h3>
        <Chart {data}/>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>


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

   .top-right {
    position: absolute;
    top: 7rem;
    right: 5.5rem;
  }
  
</style>
