<script>
  import PieChart from "$lib/Components/PieChart.svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { API_URL_GRID, API_URL_MARKET, API_URL_AGENT } from "$lib/config.js";
  import ConsumptionCurve from "$lib/Components/ConsumptionCurve.svelte";
  import ProductionCurve from "$lib/Components/ProductionCurve.svelte";

  let selectednode = "";
  let selectedAppliances = ["asdf", "jkl", "oiu"]; //by default should be all of them
  let appliances = ["asdf", "jkl", "oiu"];
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
  let marketpiedata;
  let consumptioncurvedata = [];
  let unitsproduced;
  let unitsconsumed;

  onMount(async () => {
    await getNodes();
    await getSellStats();
    await getBuyStats();
    await getBoughtSold();
    // await getBuyHistory();
    // await getSellHistory();
    // await getConsumedProduced();
    // await getCurve();
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
      selectedAppliances = [...selectedAppliances, node];
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
        maxbuy = fdata.data.max_price;
        minbuy = fdata.data.min_price;
        avgbuy = fdata.data.average_price;
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
        maxsell = fdata.data.max_price;
        minsell = fdata.data.min_price;
        avgsell = fdata.data.average_price;
      }
    } catch (error) {
      console.log(
        "An error occurred while fetching user_sell_stats data..\n",
        error
      );
    }
  }

  async function getBuyHistory() {
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
          time_frame: Day1,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
    } catch (error) {
      console.log(
        "An error occurred while fetching buy_history_stat data..\n",
        error
      );
    }
  }

  async function getSellHistory() {
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
          time_frame: Day1,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);
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
          email: email,
          node_id: nodeid,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      console.log(fdata);

      if (fdata.message == "Here is the detail") {
        let temp = fdata.consumption;
        consumption.forEach((item, index) => {
          if (!consumptioncurvedata[index]) {
            consumptioncurvedata[index] = 0;
          }
          consumptioncurvedata[index] += item.data;
        });
        console.log("This is consumption curve data:", consumptioncurvedata);
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
      }
    } catch (error) {
      console.log("An error occurred while fetching getNodes data..\n", error);
    }
  }
</script>

<div class="flex">
  <div id="lhs" class="w-1/2 pr-4">
    <!-- market stats to go here -->
    <!-- {#if minbuy} -->
    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <span class="">Market Stats</span>
      <br />
      <span class="font-light">Minimum price bought at: {minbuy}</span>
      <br />
      <span class="font-light">Maximum price bought at: {maxbuy}</span>
      <br />
      <span class="font-light">Minimum price sold at: {minsell}</span>
      <br />
      <span class="font-light">Maximum price sold at: {maxsell}</span>
    </div>

    <!-- {/if} -->

    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <PieChart data={marketpiedata} />
    </div>
  </div>

  <div id="rhs" class="w-1/2">
    <div class="flex bg-base-100 rounded-2xl p-5 mt-3 h-20">
      <select
        bind:value={selectednode}
        class="select select-bordered overflow-y-auto w-1/2 focus:outline-none"
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
      <ConsumptionCurve class="w-1/2" />
    </div>
    <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 mt-3">
      <ProductionCurve />
    </div>
  </div>
</div>
