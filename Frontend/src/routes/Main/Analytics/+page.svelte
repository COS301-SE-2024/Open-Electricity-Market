<script>
import PieChart from "$lib/Components/PieChart.svelte";
import {onMount} from "svelte";
import { goto } from '$app/navigation';
import { API_URL_GRID, API_URL_MARKET, API_URL_AGENT } from '$lib/config.js';
import ConsumptionCurve from "$lib/Components/ConsumptionCurve.svelte";
import ProductionCurve from "$lib/Components/ProductionCurve.svelte";

let selectednode = ''; 
let selectedAppliances = ['asdf', 'jkl', 'oiu'];  //by default should be all of them
let appliances = ['asdf', 'jkl', 'oiu']; 
let dropdownvisible = false;

function toggleDropdown(){
    dropdownvisible = !dropdownvisible; 
}



 async function fetchAgentData() {
    // try {
    //   const response = await fetch(`${API_URL_MARKET}/get_nodes`, {
    //     method: "POST", 
    //     headers: {
    //       'Content-Type': 'application/json',
    //       'Accept': 'application/json',
    //       'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
    //     },
    //     credentials: "include", 
    //     body: JSON.stringify({
    //       limit: 10
    //     })
    //   });
   
      
    //   const fdata = await response.json();
      
  
    // } catch (error) {
    //   console.log("An error occurred while fetching agent data..\n", error);
    // }
  };


  function toggleAppliance(appliance){
        if(selectedAppliances.includes(appliance)){
            selectedAppliances = selectedAppliances.filter((n) => n!== appliance)
        }
        else{
            selectedAppliances = [...selectedAppliances, node]; 
        }
  }


</script>

<div class = "flex">

    <div id = "lhs" class = "w-1/2 pr-4">

        <!-- market stats to go here -->
        <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
            <span class = "">Market Stats</span>
            <br>
            <span class = "font-light">Minimum price bought at: </span>
            <br>
            <span class = "font-light">Maximum price sold at: </span>
        </div>


        <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
            <PieChart />
        </div>

    
    </div>

    <div id = "rhs" class = "w-1/2">

         <div class = "flex bg-base-100 rounded-2xl p-5 mt-3 h-20 ">

           
              <select bind:value={selectednode} class="select select-bordered overflow-y-auto w-1/2 focus:outline-none">
                    <option value="" disabled selected>Node</option>
                     <!-- {#each nodes as node}
                        <option value={node}>{node}</option>
                     {/each} -->
                </select>
         
          
              




                <div class=" w-1/2">
                <button class="select select-bordered w-full text-left flex items-center h-full focus:outline-none" on:click={toggleDropdown}>Select Appliances</button>

                {#if dropdownvisible}
                <div class="mt-2 w-full bg-base-100 rounded-md overflow-y-auto">
                    {#each appliances as appliance}
                    <label class="flex items-center p-2 cursor-pointer">
                        <input
                        type="checkbox"
                        class="checkbox checkbox-primary mr-2"
                        checked={selectedAppliances.includes(appliance)}
                        on:change={() => toggleAppliance(appliance)}
                        />
                        {appliance}
                    </label>
                    {/each}
                </div>
                {/if}
                </div>
                
         
        </div>

        <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3 h-80">
            <PieChart />
        </div>
        <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
            <ConsumptionCurve class = "w-1/2" />
           
        </div>
        <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
             <ProductionCurve  />
        </div>
    
    </div>

</div>






