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
  let selectedAppliances = []; //by default should be all of them
  let appliances = new Set();
  let selectedGenerators = []; 
  let generators = []; 
  let dropdownvisible = false;
  let generatordropdownvisible = false; 

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
  let generatorNames = []; 

  onMount(async () => {
    // token check and refresh
    const session = sessionStorage.getItem("Token");
    
    if (!session) {
      goto("/login")
    } else {
      const response = await fetch(`${API_URL_MARKET}/token_refresh`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });

      const fdata = await response.json();

      // console.log(fdata);
      if (!fdata.error) {
        // swap out to the new token
        sessionStorage.removeItem("Token");
        sessionStorage.setItem("Token", fdata.data.token)
      } else {
        goto("/login")
      }
    }


    await getNodes();
    await getSellStats();
    await getBuyStats();
    await getBoughtSold();

    await getBuyHistory("Day1");
    await getSellHistory("Day1");
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

  function toggleDropdownGenerators(){
    generatordropdownvisible = !generatordropdownvisible; 
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
    console.log(selectedAppliances);
  }

  function toggleGenerator(generator){
    if(selectedGenerators.includes(generator)){
      selectedGenerators = selectedGenerators.filter((n)=> n!== generator); 
    }
    else{
      selectedGenerators = [...selectedGenerators, generator]; 
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
        maxbuy = (fdata.data.max_price * 1000).toFixed(2);
        minbuy = (fdata.data.min_price * 1000).toFixed(2);
        avgbuy = (fdata.data.average_price * 1000).toFixed(2);
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
        maxsell = (fdata.data.max_price * 1000).toFixed(2);
        minsell = (fdata.data.min_price * 1000).toFixed(2);
        avgsell = (fdata.data.average_price * 1000).toFixed(2);
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
      let temp = fdata.data.consumption || [];

      appliances.clear();
      generatorNames = []; 
      //only runs this first time - selectedAppliances gets updated in toggleAppliance
      temp.forEach((item) => {
        appliances.add(item.appliance);
      });
      selectedAppliances = Array.from(appliances);

      console.log("Selected appliances are ", selectedAppliances);
      console.log("Appliances are: ", appliances);
      // alert("it makes it here")
      consumptioncurvedata = new Array(24).fill(0);
      productioncurvedata = new Array(24).fill(0); 
      let index = 0;
      if (fdata.message == "Here is the detail") {
        //console.log("gets to the first foreach");
        temp.forEach((item) => {
          if (selectedAppliances.includes(item.appliance)) {
            console.log(item.appliance);
            if (!consumptioncurvedata[index]) {
              consumptioncurvedata[index] = 0;
            }
            consumptioncurvedata[index] += item.data;
            index++;
            if (index >= 24) {
              index = 0;
            }
          }
        });
        let temp2 = fdata.data.production || [];

        generatorNames = temp2.flatMap(item => {
          let gens =  Object.keys(item[0])[0]; 
          return gens.replace(/([A-Z])/g, ' $1').trim(); 
        });
        selectedGenerators = Array.from(generatorNames); 
        //let mikindex = 0; 
        temp2.forEach((generator) => {
          let startTime = generator[2][0].start;
          let endTime = generator[2][0].end;
          let startTimeHour = Math.round(startTime / 3600);
          let endTimeHour = Math.round(endTime / 3600);
          console.log("This is start time: ", startTimeHour);
          console.log("This is end time: ", endTimeHour);
          // for (let index = 0; index < 24; index++) {
          //   productioncurvedata[index] = 0;
          // }
          for (let index2 = startTimeHour; index2 < endTimeHour; index2++) {
            productioncurvedata[index2] += generator[1];
          }
          //mikindex++; 
        });

        // alert(consumptioncurvedata); 
        // alert(productioncurvedata); 

        // console.log("This is consumption curve data:", consumptioncurvedata);
        // console.log("This is the production curve data: ", productioncurvedata);
      }
    } catch (error) {
      console.log("An error occurred while fetching getCurve data..\n", error);
    }
  }

  //getCurve function that will not reset the appliances
  async function getCurve2() {
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
      let temp = fdata.data.consumption;

      console.log("Selected appliances are ", selectedAppliances);
      console.log("Appliances are: ", appliances);
      consumptioncurvedata = [];
      productioncurvedata = [];
      let index = 0;
      if (fdata.message == "Here is the detail") {
        //console.log("gets to the first foreach");
        //check to see if all the appliances are toggled off
        if (selectedAppliances.length === 0) {
          consumptioncurvedata = new Array(24).fill(0);
          console.log("Selected appliances is empty");
          // console.log(productioncurvedata);  
          return;
        }
        temp.forEach((item) => {
          if (selectedAppliances.includes(item.appliance)) {
            console.log(item.appliance);
            if (!consumptioncurvedata[index]) {
              consumptioncurvedata[index] = 0;
            }
            consumptioncurvedata[index] += item.data;
            index++;
            if (index >= 24) {
              index = 0;
            }
          }
        });
        // let temp2 = fdata.data.production;
        // let startTime = temp2.

        // temp2.forEach((generator) => {
        //   let startTime = generator[2][0].start;
        //   let endTime = generator[2][0].end;
        //   let startTimeHour = Math.round(startTime / 3600);
        //   let endTimeHour = Math.round(endTime / 3600);
        //   console.log("This is start time: ", startTimeHour);
        //   console.log("This is end time: ", endTimeHour);
        //   for (let index = 0; index < 24; index++) {
        //     productioncurvedata[index] = 0;
        //   }
        //   for (let index2 = startTimeHour; index2 < endTimeHour; index2++) {
        //     productioncurvedata[index2] = generator[1];
        //   }
        // });
      }
    } catch (error) {
      console.log("An error occurred while fetching getCurve2 data..\n", error);
    }
  }

  //will not reset the generators
  async function getCurve3() {
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
      let temp = fdata.data.consumption;

     
      //consumptioncurvedata = [];
      // productioncurvedata = [];
      productioncurvedata = new Array(24).fill(0);
      let index = 0;
      if (fdata.message == "Here is the detail") {
        //console.log("gets to the first foreach");
        //check to see if all the appliances are toggled off
        // if (selectedAppliances.length === 0) {
        //   consumptioncurvedata = new Array(24).fill(0);
        //   console.log("Selected appliances is empty");
        //   // console.log(productioncurvedata);  
        //   return;
        // }
        // temp.forEach((item) => {
        //   if (selectedAppliances.includes(item.appliance)) {
        //     console.log(item.appliance);
        //     if (!consumptioncurvedata[index]) {
        //       consumptioncurvedata[index] = 0;
        //     }
        //     consumptioncurvedata[index] += item.data;
        //     index++;
        //     if (index >= 24) {
        //       index = 0;
        //     }
        //   }
        // });
        let temp2 = fdata.data.production;
        
        
        // let startTime = temp2.
        let tempindex = 0; 
        temp2.forEach((generator) => {
          let genInfo = generator[0]; 
          // alert(Object.keys(genInfo)); 

          let alteredSelectedGenerators = selectedGenerators.map(gen => {
            return gen.replace(/\s+/g, ""); 
          }); 
          let listOfGenerators = Object.keys(genInfo).toString();
          // alert("This is list of selected: "+ alteredSelectedGenerators.join('')); 
          // alert("This is curr gen '"+ listOfGenerators+"'"); 
          // alert(alteredSelectedGenerators.includes(listOfGenerators)); 
          if(alteredSelectedGenerators.includes(listOfGenerators)){
              //alert("The if condition was met"); 
          let startTime = generator[2][0].start;
          let endTime = generator[2][0].end;
          let startTimeHour = Math.round(startTime / 3600);
          let endTimeHour = Math.round(endTime / 3600);
          console.log("This is start time: ", startTimeHour);
          console.log("This is end time: ", endTimeHour);
          // for (let index = 0; index < 24; index++) {
          //   productioncurvedata[index] += 0;
          // }
          for (let index2 = startTimeHour; index2 < endTimeHour; index2++) {
            productioncurvedata[index2] += generator[1];
          }
          }
        });
      }
    } catch (error) {
      console.log("An error occurred while fetching getCurve3 data..\n", error);
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
        agentpiedata = { unitsconsumed, unitsproduced };
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
        selectednode = listofnodes.length > 0 ? listofnodes[0] : "";
        console.log(listofnodes);
        console.log(listofnodeids);
        nodeid = listofnodeids[0];
        // document.getElementById("nodeselector").selected = listofnodes[0];
      }
    } catch (error) {
      console.log("An error occurred while fetching getNodes data..\n", error);
    }
  }

  function updateAllAgent() {
    productioncurvedata = new Array(24).fill(0); 
    consumptioncurvedata = new Array(24).fill(0); 
    getCurve();
    getConsumedProduced();
  }

  function updateAllAgent2() {
    getCurve2();
    getConsumedProduced();
  }

  function updateAllAgent3(){
    getCurve3(); 
  }

  function updateNode() {
    //function that updates nodeid before agent endpoints get called
    let currindex = listofnodes.indexOf(selectednode);
    nodeid = listofnodeids[currindex];
  }
</script>

<div class="md:flex xs:flex-row">
  <div id="lhs" class="md:w-1/2 md:pr-4 xs:w-1/1 ">
    <!-- market stats to go here -->
    <!-- {#if minbuy} -->
    <span class="text-3xl font-thin justify-start pl-2">
      Market Stats
    </span>
    <div class="flex flex-row min-w-3/4 bg-base-100 rounded-2xl p-3 mt-3 overflow-auto">
      <div class="card bg-gradient-to-b from-blue-400 to-primary mr-2">
        <span class="font-normal card bg-blue-700 p-2 text-white rounded-sm rounded-t-xl justify-center text-center">
          Minimum <br> buy price:
        </span>
        <span class="font-normal text-white text-4xl p-3 pt-2">R{minbuy}</span>
      </div>
      <!-- <div class="card bg-gradient-to-b from-blue-400 to-primary p-5 mx-2">
        <span class="font-normal">
          Maximum BUY: 
        </span>
        <span class="font-normal text-5xl">R{maxbuy}</span>
      </div>
      <div class="card bg-gradient-to-b from-blue-500 to-blue-600 p-5 mx-2">
        <span class="font-normal">
          Average BUY: 
        </span>
        <span class="font-normal text-5xl">R{avgbuy}</span>
      </div> -->
      <div class="card bg-gradient-to-b from-blue-400 to-primary mr-2">
        <span class="font-normal card bg-blue-700 p-2 text-white rounded-sm rounded-t-xl justify-center text-center">
          Maximum <br>buy price:
        </span>
        <span class="font-normal text-white text-4xl p-3 pt-2">R{maxbuy}</span>
      </div>
      <div class="card bg-gradient-to-b from-blue-400 to-primary mr-2">
        <span class="font-normal card bg-blue-700 p-2 text-white rounded-sm rounded-t-xl justify-center text-center">
          Average <br> buy price:
        </span>
        <span class="font-normal text-white text-4xl p-3 pt-2">R{avgbuy}</span>
      </div>
      <!-- previous attempts -->
      <!-- <div class="card bg-gradient-to-b from-orange-400 to-orange-600 p-5 mx-2">
        <span class="font-normal">
          Minimum SELL: 
        </span>
        <span class="font-normal text-5xl">R{minsell}</span>
      </div>
      <div class="card bg-gradient-to-b from-orange-400 to-orange-600 p-5 mx-2">
        <span class="font-normal">
          Maximum SELL: 
        </span>
        <span class="font-normal text-5xl">R{maxsell}</span>
      </div>
      <div class="card bg-gradient-to-b from-orange-400 to-orange-600 p-5 ml-2">
        <span class="font-normal">
          Average SELL: 
        </span>
        <span class="font-normal text-5xl">R{avgsell}</span>
      </div> -->
      <div class="card bg-gradient-to-b from-orange-400 to-orange-600 mr-2">
        <span class="font-normal card bg-orange-600 p-2 text-white rounded-sm rounded-t-xl justify-center text-center">
          Minimum <br> sell price:
        </span>
        <span class="font-normal text-white text-4xl p-3 pt-2">R{minsell}</span>
      </div>
      <div class="card bg-gradient-to-b from-orange-400 to-orange-600 mr-2">
        <span class="font-normal card bg-orange-600 p-2 text-white rounded-sm rounded-t-xl justify-center text-center">
          Maximum <br> sell price:
        </span>
        <span class="font-normal text-white text-4xl p-3 pt-2">R{maxsell}</span>
      </div>
      <div class="card bg-gradient-to-b from-orange-400 to-orange-600 mr-2">
        <span class="font-normal card bg-orange-600 p-2 text-white rounded-sm rounded-t-xl justify-center text-center">
          Average <br> sell price:
        </span>
        <span class="font-normal text-white text-4xl p-3 pt-2">R{avgsell}</span>
      </div>
      {#if marketpiedata.length >= 1}
        <div class="w-1/2 mr-16">
          <PieChart {marketpiedata} />
        </div>
      {/if}
    </div>

    <!-- {/if} -->

    <!-- <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <PieChart {marketpiedata} />
    </div> -->

    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      
      <div class="form-control">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <select
          bind:value={buyChartPeriod}
          class="select select-bordered max-h-40 overflow-y-auto"
          on:change={() => getBuyHistory(buyChartPeriod)} 
        >
          <option value="Day1" default selected>24h</option>
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
          <option value="Day1" default selected>24h</option>
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

  <div id="rhs" class="xs:mt-6 md:mt-0">
    <span class="text-3xl font-thin justify-start pl-2 xs:mt-4 md:mt-0">
      Node Stats
    </span>
    <div class="md:flex sm:flex-col md:flex-row bg-base-100 rounded-2xl p-3 mt-3 xs:h-50">
      <div class="md:w-1/3 md:pr-2">
        <select
          bind:value={selectednode}
          class="select select-bordered w-full overflow-y-auto mr-2 focus:outline-none "
          on:change={() => {
            updateNode();
            updateAllAgent();
          }}
        >
          <!-- <option value="" disabled id = "nodeselector">Select Node</option> -->
          {#each listofnodes as node}
            <option value={node}>{node}</option>
          {/each}
        </select>
      </div>
      <div class="md:w-1/3 md:px-2 md:py-0 py-2">
        <button
          class="select select-bordered w-full text-left flex items-center focus:outline-none z-9000"
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
                  on:change={() => {
                    toggleAppliance(appliance);
                    updateAllAgent2();
                  }}
                />
                {appliance}
              </label>
            {/each}
          </div>
        {/if}
      </div>
      <div class="md:w-1/3 md:pl-2">
        <button
          class="select select-bordered w-full text-left flex items-center focus:outline-none z-9000"
          on:click={toggleDropdownGenerators}>Select Generators</button
        >

        {#if generatordropdownvisible}
          <div class="mt-2 w-full bg-base-100 rounded-md overflow-y-auto">
            {#each generatorNames as generator}
              <label class="flex items-center p-2 cursor-pointer">
                <input
                  type="checkbox"
                  class="checkbox checkbox-primary mr-2"
                  checked={selectedGenerators.includes(generator)}
                  on:change={() => {
                    toggleGenerator(generator);
                    updateAllAgent3();
                  }}
                />
                {generator}
              </label>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- {#if agentpiedata.length>1} -->
    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3 md:h-80 md:h-40">
      <PieChartAgent {agentpiedata} />
    </div>
    <!-- {/if} -->
    <div class="md:flex xs:flex-row md:min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <ConsumptionCurve class="md:w-1/2" data={consumptioncurvedata} />
      <ProductionCurve class="md:ml-8 md:w-1/2 xs:ml-0" data={productioncurvedata} />
    </div>
    <!-- <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
    
    </div> -->
  </div>
</div>
