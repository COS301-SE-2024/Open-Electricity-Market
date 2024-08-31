<script>
import Chart from "$lib/Components/Chart.svelte";
import {onMount} from "svelte";
import { goto } from '$app/navigation';
import { API_URL_GRID, API_URL_MARKET } from '$lib/config.js';
import { createChart } from 'lightweight-charts';
import PriceChartD3 from "$lib/Components/PriceChartD3.svelte";
// ***********************************************




// **********************************************
let selectedPrice = 0;
$: price = 0;
let units = 1;
let chartPeriod; 

let selected_node_id = sessionStorage.getItem("node_id");
let selected_node_name = sessionStorage.getItem("node_name");

let data = [];

async function place_buy_order(at_market_price) {
  // TODO: add a check that fails if units <= 0

  if (at_market_price == true) {
    selectedPrice = price;
  }

  let data = {
    "node_id": selected_node_id,
    "min_price": selectedPrice > 1 ? selectedPrice - 1 : 0.01,
    "max_price": selectedPrice + 1,
    "units": units
  }

  const response = await fetch(`${API_URL_MARKET}/buy_order`, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
    },
    body : JSON.stringify(data),
    credentials: 'include'
  });

  goto('../Main/Dashboard');
}

async function place_sell_order(at_market_price) {
  // TODO: add a check that fails if units <= 0

  if (at_market_price == true) {
    selectedPrice = price;
  }

  let data = {
    "node_id": selected_node_id,
    "min_price": selectedPrice > 1 ? selectedPrice - 1 : 0.01,
    "max_price": selectedPrice + 1,
    "units": units
  }

  const response = await fetch(`${API_URL_MARKET}/sell_order`, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
    },
    body : JSON.stringify(data),
    credentials: 'include'
  });

  goto('../Main/Dashboard');
}

onMount(async () => {
  await fetchPriceHistory();
  let interval = setInterval(fetchPriceHistory, 2000);
  
  selectedPrice = price;

  //return function runs when the component is unmounted
  return() => {
    clearInterval(interval);
  };
});

async function fetchData() {

  try {
    const response = await fetch(`${API_URL_MARKET}/price_view`, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
      }

    });

    const fdata = await response.json();

    data = fdata.data
    // console.log(data)

    price = data.price;

  } catch (error) {
    console.log("There was an error fetching the JSON for the overview..", error);
  }
}


 async function fetchPriceHistory(){

    try {
      const response = await fetch(`${API_URL_MARKET}/price_history`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
        credentials: "include", 
        body: JSON.stringify({
          hours: 48
        })
      });
      
      const fdata = await response.json();
      console.log(fdata); 
      data = fdata.data.map(item => parseFloat(item.price.toFixed(2)));
      price = data.length > 0 ? data[data.length - 1] : 0;
      console.log("This is data for the chart: " + data); 
      
      
      
      

      
    } catch (error) {
      console.log("An error occurred while fetching price history", error);
    }

  };


</script>

<main class="container mx-auto p-4">
  <div class="md:flex md:flex-row">
    <div class="md:basis-2/3 bg-base-100 md:card md:mr-5 md:p-4">
      <h1 class="md:text-5xl md:font-bold md:pt-8">Marketplace</h1>
       <div class="form-control">
          <!-- svelte-ignore a11y-label-has-associated-control -->
          <select bind:value={chartPeriod} class="select select-bordered max-h-40 overflow-y-auto">
              <option value="" disabled selected></option>
                  <option value="1">1h</option> 
                  <option value="24">24h</option>
                  <option value="168">1w</option>
          </select>
      </div>
      <Chart {data}  />
       <!-- <PriceChartD3 id = "chartPrice" />  -->
    </div>
    <div class="md:basis-1/3 md:card bg-base-100 md:p-4 xs:pt-10">
      <span class="text-lg">Selected Node: </span> <br>
      <span class="text-3xl">{selected_node_name}</span> <br>
      <span class="text-lg">Current Average Market Price: </span> <br>
      <span class="text-3xl">R {price.toFixed(2)}</span> <br>
      
      <form>
        <div class="form-control mt-1">
          <label for="buy_price"> Price </label>
          <input id="buy_price" type="number" placeholder="{selectedPrice}" class="input input-bordered" name="buy_price" required bind:value={selectedPrice}/>
        </div>

        <div class="form-control mt-1">
          <label for="amount"> Number of units </label>
          <input id="buy_units" type="number" placeholder="{units}" class="input input-bordered" name="amount" required bind:value={units}/>
        </div>

        <div class="mt-1 xs:pt-5">
          <button class="btn btn-primary" onclick="my_modal_1.showModal()">Buy</button>
          <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">Please confirm your buy order for {units} units at R{selectedPrice.toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn btn-secondary" on:click={() => place_buy_order(false)} >Continue</button>
                  <button class="btn btn-error">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <button class="btn btn-primary" onclick="my_modal_3.showModal()">Buy at Market Price</button>
          <dialog id="my_modal_3" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">Please confirm your buy order for {units} units at R{price.toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn btn-secondary" on:click={() => place_buy_order(true)} >Continue</button>
                  <button class="btn btn-error">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <button class="btn btn-accent" onclick="my_modal_2.showModal()">Sell</button>
          <dialog id="my_modal_2" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Sell Order</h3>
              <p class="py-4">Please confirm your sell order for {units} units at R{selectedPrice.toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn btn-secondary" on:click={() => place_sell_order(false)}>Continue</button>
                  <button class="btn btn-error">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <button class="btn btn-accent" onclick="my_modal_4.showModal()">Sell at Market Price</button>
          <dialog id="my_modal_4" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Sell Order</h3>
              <p class="py-4">Please confirm your sell order for {units} units at R{price.toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn btn-secondary" on:click={() => place_sell_order(true)}>Continue</button>
                  <button class="btn btn-error">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <!-- <button class="btn btn-success" onclick="my_modal_1.showModal()">Buy at market price</button> -->
        </div>

      </form>

    </div>
  </div>
</main>



<style>
</style>
