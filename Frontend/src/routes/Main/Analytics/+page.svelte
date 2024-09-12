<script>
  import PieChart from "$lib/Components/PieChart.svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { API_URL_GRID, API_URL_MARKET, API_URL_AGENT } from "$lib/config.js";
  import ConsumptionCurve from "$lib/Components/ConsumptionCurve.svelte";
  import ProductionCurve from "$lib/Components/ProductionCurve.svelte";
  import PriceHistoryChart from "$lib/Components/PriceHistoryChart.svelte";
  import PieChartAgent from "$lib/Components/PieChartAgent.svelte";

  let selectednode = "";
  let selectedAppliances = ["asdf", "jkl", "oiu"]; //by default should be all of them
  let appliances = new Set(); 
  let dropdownvisible = false;

  //required for curve endpoint
  let email = "";
  let nodeid = "";
  let nodes;
  let listofnodes = [];
  let maxbuy;
  let minbuy;
  let avgbuy;
  let maxsell;
  let minsell;
  let avgsell;
  let unitsbought;
  let unitssold;
  let listofnodeids = [];
  let marketpiedata = {};
  let agentpiedata = {}; 
  let consumptioncurvedata = [];
  let unitsproduced;
  let unitsconsumed;
  let productioncurvedata = []; 
  let buyChartPeriod; 
  let sellChartPeriod; 
  let buyhistorydata = []; 
  let sellhistorydata = [];  

  

  onMount(async () => {
    await getNodes();
    await getSellStats();
    await getBuyStats();
    await getBoughtSold();

    await getBuyHistory();
    await getSellHistory();
    await getConsumedProduced();
    await getCurve(); 

    console.log("Appliances are: ", appliances); 
    console.log("Selected Appliances are: ", selectedAppliances);
    console.log("Units produced are: ", unitsproduced);
    console.log("Units consumed are: ", unitsconsumed);   
    
  });

  function toggleDropdown() {
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
  }

  function toggleAppliance(appliance) {
    if (selectedAppliances.includes(appliance)) {
      selectedAppliances = selectedAppliances.filter((n) => n !== appliance);
    } else {
      selectedAppliances = [...selectedAppliances, appliance];
    }
  }

  async function getBuyStats() {
    try {
      const response = await fetch(`${API_URL_MARKET}/user_buy_stats`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
      if (fdata.message == "User's buying stats successfully retrieved") {
        maxbuy = fdata.data.max_price.toFixed(2);
        minbuy = fdata.data.min_price.toFixed(2);
        avgbuy = fdata.data.average_price.toFixed(2);
      }
    } catch (error) {
      console.log(
        "An error occurred while fetching user_buy_stats data..\n",
        error
      );
    }
  }

  async function getSellStats() {
    try {
      const response = await fetch(`${API_URL_MARKET}/user_sell_stats`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
      if (fdata.message == "User's selling stats successfully retrieved") {
        maxsell = fdata.data.max_price.toFixed(2);
        minsell = fdata.data.min_price.toFixed(2);
        avgsell = fdata.data.average_price.toFixed(2);
      }
    } catch (error) {
      console.log(
        "An error occurred while fetching user_sell_stats data..\n",
        error
      );
    }
  }

  async function getBuyHistory(chartPeriod) {
    try {
      const response = await fetch(`${API_URL_MARKET}/buy_history_stat`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          //options include Day1, Week1, Month1, Month3, Month6, Year1
          time_frame: chartPeriod,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
      buyhistorydata = fdata.data.map((item) =>
        parseFloat(item.price.toFixed(2))
      );
    } catch (error) {
      console.log(
        "An error occurred while fetching buy_history_stat data..\n",
        error
      );
    }
  }

  async function getSellHistory(chartPeriod) {
    try {
      const response = await fetch(`${API_URL_MARKET}/sell_history_stat`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          //options include Day1, Week1, Month1, Month3, Month6, Year1
          time_frame: chartPeriod,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
      sellhistorydata = fdata.data.map((item) =>
        parseFloat(item.price.toFixed(2))
      );
    } catch (error) {
      console.log(
        "An error occurred while fetching sell_history_stat data..\n",
        error
      );
    }
  }

  async function getBoughtSold() {
    try {
      const response = await fetch(`${API_URL_MARKET}/bought_vs_sold_stat`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
      if (
        fdata.message == "Successfully retrieved user bought and sold units"
      ) {
        unitsbought = fdata.data.units_bought;
        unitssold = fdata.data.units_sold;
        marketpiedata = { unitsbought, unitssold };
        // console.log(marketpiedata);
      }
    } catch (error) {
      console.log(
        "An error occurred while fetching bought_vs_sold data..\n",
        error
      );
    }
  }

  async function getCurve() {
    
    console.log("curve was running");
    console.log(sessionStorage.getItem("email")); 
    try {
      
      const response = await fetch(`${API_URL_AGENT}/get_curve`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          email: sessionStorage.getItem("email"),
          node_id: nodeid,
        }),
        credentials: "include",
      });
      
      
      const fdata = await response.json();
      console.log(fdata);
      let temp = fdata.consumption;

      if(selectedAppliances[0]=="asdf"){
          //only runs this first time - selectedAppliances gets updated in toggleAppliance
            temp.forEach((item) => {
              appliances.add(item.appliance); 
          });
          selectedAppliances = Array.from(appliances);
          
        }
        console.log("Selected appliances are ", selectedAppliances);
        console.log("Appliances are: ", appliances); 

      let index = 0; 
      if (fdata.message == "Here is the detail") {
        console.log("gets to the first foreach"); 
        temp.forEach((item) => {
         
          if(selectedAppliances.includes(item.appliance)){
            if (!consumptioncurvedata[index]) {
              consumptioncurvedata[index] = 0;
            }
            consumptioncurvedata[index] += item.data;
            index++; 
            if(index >= 24){
              index = 0; 
            }

          }
  
        });
        let temp2 = fdata.production;

        temp2.forEach((generator) => {
            //should be the value
            // productioncurvedata = temp2[1];
            for(let index = 0; index<24; index++){
                productioncurvedata[index] = generator[1]; 
            } 
        });

       

        

        

        console.log("This is consumption curve data:", consumptioncurvedata);
        console.log("This is the production curve data: ", productioncurvedata); 
      }
    } catch (error) {
      console.log("An error occurred while fetching getCurve data..\n", error);
    }
  }

  async function getConsumedProduced() {
    try {
      const response = await fetch(`${API_URL_AGENT}/get_consumed_produced`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          node_id: nodeid,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
      if (fdata.message == "Here is the detail") {
        unitsconsumed = fdata.data.consumed;
        unitsproduced = fdata.data.produced;
        agentpiedata = {unitsconsumed, unitsproduced}; 

      }
    } catch (error) {
      console.log(
        "An error occurred while fetching getConsumedProduced data..\n",
        error
      );
    }
  }

  async function getNodes() {
    try {
      const response = await fetch(`${API_URL_MARKET}/get_nodes`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          limit: 10,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
      if (fdata.message == "List of nodes successfully retrieved") {
        nodes = fdata.data;
        listofnodes = fdata.data.map((nodes) => nodes.name);
        listofnodeids = fdata.data.map((nodes) => nodes.node_id);
        console.log(listofnodes);
        console.log(listofnodeids);
        nodeid = listofnodeids[0]; 
        
      }
    } catch (error) {
      console.log("An error occurred while fetching getNodes data..\n", error);
    }
  }


  function updateAllAgent(){
    getCurve(); 
    getConsumedProduced(); 
  }

  function updateNode(){
    //function that updates nodeid before agent endpoints get called
    let currindex = listofnodes.indexOf(selectednode);
    nodeid = listofnodeids[currindex]; 
  }




  
</script>

<div class="flex">
  <div id="lhs" class="w-1/2 pr-4">
    <!-- market stats to go here -->
    <!-- {#if minbuy} -->
    <span class="text-3xl text-white font-thin justify-start pl-2">
      Market Stats
    </span>
    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <!-- <span class="">Market Stats</span>
      <br /> -->
      <span class="font-light"
        >Minimum price bought at: <span class="font-normal">R{minbuy}</span
        ></span
      >
      <br />
      <span class="font-light"
        >Maximum price bought at: <span class="font-normal">R{maxbuy}</span
        ></span
      >
      <br />
      <span class="font-light"
        >Average price bought at: <span class="font-normal">R{avgbuy}</span
        ></span
      >
      <br />
      <span class="font-light"
        >Minimum price sold at: <span class="font-normal">R{minsell}</span
        ></span
      >
      <br />
      <span class="font-light"
        >Maximum price sold at: <span class="font-normal">R{maxsell}</span
        ></span
      >
      <br />
      <span class="font-light"
        >Average price sold at: <span class="font-normal">R{avgsell}</span
        ></span
      >
    </div>

    <!-- {/if} -->

    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <PieChart {marketpiedata} />
    </div>

    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <div class="form-control">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <select
          bind:value={buyChartPeriod}
          class="select select-bordered max-h-40 overflow-y-auto"
          on:change={() => getBuyHistory(buyChartPeriod)}
        >
          <option value="Day1" default>24h</option>
          <option value="Week1">7d</option>
          <option value="Month1">1M</option>
          <option value="Month3">3M</option>
          <option value="Month6">6M</option>
          <option value="Year1">1Y</option>
        </select>
      </div>
      <PriceHistoryChart class="w-1/2" data={buyhistorydata} />
    </div>

    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <div class="form-control">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <select
          bind:value={sellChartPeriod}
          class="select select-bordered max-h-40 overflow-y-auto"
          on:change={() => getSellHistory(sellChartPeriod)}
        >
          <option value="Day1" default>24h</option>
          <option value="Week1">7d</option>
          <option value="Month1">1M</option>
          <option value="Month3">3M</option>
          <option value="Month6">6M</option>
          <option value="Year1">1Y</option>
        </select>
      </div>
      <PriceHistoryChart class="w-1/2" data={sellhistorydata} />
    </div>
  </div>


  
  <div id="rhs" class="w-1/2">
    <span class="text-3xl text-white font-thin justify-start pl-2">
      Node Stats
    </span>
    <div class="flex bg-base-100 rounded-2xl p-5 mt-3 h-20">
      <select
        bind:value={selectednode}
        class="select select-bordered overflow-y-auto w-1/2 focus:outline-none"
        on:change={()=>{updateNode(); updateAllAgent();}}
      >
        <option value="" disabled selected>Select Node</option>
        {#each listofnodes as node}
          <option value={node}>{node}</option>
        {/each}
      </select>

      <div class=" w-1/2">
        <button
          class="select select-bordered w-full text-left flex items-center h-full focus:outline-none"
          on:click={toggleDropdown}>Select Appliances</button
        >

        {#if dropdownvisible}
          <div class="mt-2 w-full bg-base-100 rounded-md overflow-y-auto">
            {#each appliances as appliance}
              <label class="flex items-center p-2 cursor-pointer">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary mr-2"
                  checked={selectedAppliances.includes(appliance)}
                  on:change={() => {toggleAppliance(appliance); updateAllAgent()}}
                />
                {appliance}
              </label>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3 h-80">
      <PieChartAgent {agentpiedata} />
    </div>
    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <ConsumptionCurve class="w-1/2" data = {consumptioncurvedata} />
    </div>
    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <ProductionCurve data = {productioncurvedata} />
    </div>
  </div>
</div>
