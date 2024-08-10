

<script>
  import { onMount } from "svelte";
  import Map from '$lib/Components/Map.svelte';
  import Chart from "$lib/Components/Chart2.svelte";
  import GridStats from "../../../lib/Components/GridStats.svelte";
  import { API_URL_GRID, API_URL_MARKET } from '$lib/config.js';


 

  let data = {};
  let interval; 
  let numDecimals = 2; 
  let advancedView = false; 
  let dropdownViewable = false; 
  let mapdata; 
  let oscilloscopedata = null;

  function toggleDropdown(){
    dropdownViewable = !dropdownViewable; 
  }

  onMount(async () => {
    await fetchData();
    await fetchstart();  
    //interval = setInterval(fetchData, 10000); 
    
   
    return () => {
      clearInterval(interval);
    };
  });

     async function fetchstart() {

      try {
        const response = await fetch(`${API_URL_GRID}/start`, {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
        console.log("start being sent...");
        // const response = fetch("http://localhost:8000");
        const startdata = await response.json();
        console.log(startdata);
        //Voltage 1,2,3 as well as price
        //updateChart(data.Phase1, data.Phase2);
      } catch (error) {
        console.log("There was an error fetching the JSON for the chart..", error);
        
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

  async function fetchData() {
    try {
      const response = await fetch(`${API_URL_GRID}/info`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        }
      });
      //console.log("Request being sent...");
      const fdata = await response.json();
      console.log("Fetched data:", fdata);
      mapdata = fdata.circuits[0];
      // data = null; 
      // data = {
      //   ...fdata,
      //   Consumers: fdata.Consumers.map(item => JSON.parse(item)),
      //   Generators: fdata.Generators.map(item => JSON.parse(item)),
      //   Transformers: fdata.Transformers.map(item => JSON.parse(item)),
      //   "Transmission Lines": fdata["Transmission Lines"].map(item => JSON.parse(item))
      // };
      // console.log("Data is this: ");
      // console.log(data);
      // chartdata = data[Consumers.Voltage.Phase1];
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the overview:", error);
    }
  }


  function handleMarkerClick(entity){
    // console.log(entity.detail.voltage);
    // data = entity.detail; 
    const markerData = entity.detail;
    console.log('Marker clicked:', markerData);  
    data = { ...markerData.voltage };
    console.log("Updated data is this: ", data);
  }

</script>

<main class="container mx-auto p-4">

 <!-- <div class="dropdown mr-3 mt-3">
  <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6a1.5 1.5 0 110 3 1.5 1.5 0 010-3zm0 4.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zm0 4.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3z" />
    </svg>
  </div>
  svelte-ignore a11y-no-noninteractive-tabindex
   <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
      <li class="flex items-center">
      <label class="cursor-pointer flex items-center space-x-3 w-full hover:bg-gray-600">
        <input type="checkbox" class="checkbox checkbox-primary" on:click={setAdvancedView} />
        <span>Advanced view</span>
      </label>
    </li>
  </ul>
</div> -->


<!-- <button class="btn" onclick="my_modal_2.showModal()">Help</button> -->
    <!-- <dialog id="my_modal_2" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Grid Simulation Page</h3>
        <p class="py-4">The grid simulation page contains an overview of the current 
          state of the electrical grid. 
        </p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog> -->

  <!-- <div class="form-control top-right">
  <label class="label cursor-pointer">
    <span class="label-text mr-2">Advanced view</span>
    <input type="checkbox" class="toggle" checked={advancedView} on:change={setAdvancedView} />
  </label>
  </div> -->

   
 
  <!-- {#if Object.keys(data).length > 0}
    
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Consumers</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data.Consumers as consumer}
          <div class="card shadow-md p-4 transform transition duration-200 hover:scale-105 hover:shadow-lg">
            <h3 class="text-lg font-semibold mb-2">Consumer ID: {consumer.ID}</h3>
            <p><strong>Connected Transmission Line:</strong> {consumer["Connected Transmission Line"]}</p>
            <p><strong>Resistance:</strong> {consumer.Resistance} Ω</p>
            <p><strong>Phase 1 Voltage:</strong> {consumer.Voltage["Phase 1"].toFixed(numDecimals)} V</p>
            <p><strong>Phase 2 Voltage:</strong> {consumer.Voltage["Phase 2"].toFixed(numDecimals)} V</p>
            <p><strong>Phase 3 Voltage:</strong> {consumer.Voltage["Phase 3"].toFixed(numDecimals)} V</p>
          </div>
        {/each}
      </div>
    </section>

    
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Generators</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data.Generators as generator}
          <div class="card shadow-md p-4 transform transition duration-300 hover:scale-105 hover:shadow-lg ">
            <h3 class="text-lg font-semibold mb-2">Generator ID: {generator.ID}</h3>
            <p><strong>Connected Transmission Line:</strong> {generator["Connected Transmission Line"]}</p>
            <p><strong>Frequency:</strong> {generator.Frequency} Hz</p>
            <p><strong>Max Voltage:</strong> {generator["Max Voltage"]} V</p>
            <p><strong>Phase 1 Voltage:</strong> {generator.Voltage["Phase 1"].toFixed(numDecimals)} V</p>
            <p><strong>Phase 2 Voltage:</strong> {generator.Voltage["Phase 2"].toFixed(numDecimals)} V</p>
            <p><strong>Phase 3 Voltage:</strong> {generator.Voltage["Phase 3"].toFixed(numDecimals)} V</p>
          </div>
        {/each}
      </div>
    </section>

  
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Transformers</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data.Transformers as transformer}
          <div class="card shadow-md p-4 transform transition duration-300 hover:scale-105 hover:shadow-lg">
            <h3 class="text-lg font-semibold mb-2">Transformer ID: {transformer.ID}</h3>
            <p><strong>Primary Transmission Line:</strong> {transformer["Primary Transmission Line"]}</p>
            <p><strong>Secondary Transmission Line:</strong> {transformer["Secondary Transmission Line"]}</p>
            <p><strong>Ratio:</strong> {transformer.Ratio}</p>
          </div>
        {/each}
      </div>
    </section>

   
    <section class="mb-8">
      <h2 class="text-2xl font-bold mb-4">Transmission Lines</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each data["Transmission Lines"] as line}
          <div class="card shadow-md p-4 transform transition duration-300 hover:scale-105 hover:shadow-lg ">
            <h3 class="text-lg font-semibold mb-2">Transmission Line ID: {line.ID}</h3>
            <p><strong>Impedance:</strong> {line.Impedance} Ω</p>
            <p><strong>Resistance:</strong> {line.Resistance} Ω</p>
            <p><strong>Phase 1 Voltage:</strong> {line.Voltage["Phase 1"].toFixed(numDecimals)} V</p>
            <p><strong>Phase 2 Voltage:</strong> {line.Voltage["Phase 2"].toFixed(numDecimals)} V</p>
            <p><strong>Phase 3 Voltage:</strong> {line.Voltage["Phase 3"].toFixed(numDecimals)} V</p>
          </div>
        {/each}
      </div>
    </section>
  {:else}
   <span class="loading loading-ring loading-lg ml-6"></span>
  {/if} -->

<div class="fullsection flex md:flex-row xs:flex-col">

<div class="mapsection md:w-3/5  xs:w-full xs:p-0 left-0">
  <Map {mapdata} on:markerClick = {handleMarkerClick} class="xs:rounded-md"  /> 

    <div class="statsection">
        <GridStats /> 
        </div>
    </div>

<div class="chartsection md:w-2/5 md:h-full p-5 xs:w-full xs:">   
  <Chart {data} />

      
</div>





</div>






</main>

<style>

  
</style>

      
