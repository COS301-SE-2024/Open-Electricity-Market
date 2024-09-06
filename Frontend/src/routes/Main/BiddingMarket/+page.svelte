<script>
import Chart from "$lib/Components/Chart.svelte";
import {onMount} from "svelte";
import { goto } from '$app/navigation';
import { API_URL_GRID, API_URL_MARKET } from '$lib/config.js';
import { createChart } from 'lightweight-charts';
import PriceChartD3 from "$lib/Components/PriceChartD3.svelte";
// ***********************************************




// **********************************************
$: selectedPrice = 0;
$: price = 0;
let units = 1;

let selected_node_id = sessionStorage.getItem("node_id");
let selected_node_name = sessionStorage.getItem("node_name");

let data = {};
async function reset_price(){
  // console.log("setting price to " + price);
  selectedPrice = price;
}

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
  fetchData();
  let interval = setInterval(fetchData, 2000);

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


</script>

<main class="container mx-auto p-4">
  <div class="md:flex md:flex-row">
    <div class="md:basis-2/3 bg-base-100 md:card md:mr-5 md:p-4">
      <h1 class="md:text-5xl md:font-light md:pt-8">Marketplace</h1>
      <!-- <Chart {data} class = "" /> -->
       <PriceChartD3  /> 
    </div>
    <div class="md:basis-1/3 md:card bg-base-100 md:p-4 xs:pt-10">
      <h1 class = "md:text-4xl md:font-light md:pt-4">Node Info</h1>
      <hr>
      <br>
      <span class="text-lg font-light">Selected Node: </span>
      <span class="text-3xl">{selected_node_name}</span> <br>
      <hr>
      <br>
      <span class="text-lg font-light">Current Average Market Price: </span>
      <span class="text-3xl">R {price.toFixed(2)}</span> <br>
      <hr>
      <br>
      <div>
        <div class="form-control mt-1">
          <label for="buy_price" class = "font-light"> Price </label>
          <div class = "flex">
            <input id="buy_price" type="number" placeholder="{selectedPrice}" class="basis-2/3 input input-bordered font-bold" name="buy_price" required bind:value={selectedPrice}/>
            <span class = "md:p-1">
            
            </span>
            <button class = "basis-1/4 btn btn-primary font-light" title = "Resets price back to current average market price" on:click={reset_price}>Market price</button>
          </div>
        </div>
        <br>
        <hr>
        <br>
        <div class="form-control mt-1">
          <label for="amount" class = "font-light"> Watt-hours </label>
          <input id="buy_units" type="number" placeholder="{(units == null ? 1 : units)}" class="input input-bordered font-bold" name="amount" required bind:value={units}/>
        </div>
        <br>
        <hr>
        <div class="mt-1 xs:pt-5 flex">
          <button class="md:basis-1/2 btn btn-primary font-light" onclick="my_modal_1.showModal()">Buy</button>
          <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">Please confirm your buy order for {(units == null ? 1 : units)} units at R{(selectedPrice == null ? selectedPrice = price : selectedPrice).toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn bg-green-600" on:click={() => place_buy_order(false)} >Continue</button>
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <!--<button class="btn btn-primary font-light" onclick="my_modal_3.showModal()">Buy at Market Price</button>
          <dialog id="my_modal_3" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">Please confirm your buy order for {units} units at R{price.toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn bg-green-600" on:click={() => place_buy_order(true)} >Continue</button>
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>-->
          <span class = "xs:p-1">
            
          </span>

          <button class="md:basis-1/2 btn btn-accent font-light" onclick="my_modal_2.showModal()">Sell</button>
          <dialog id="my_modal_2" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Sell Order</h3>
              <p class="py-4">Please confirm your sell order for {(units == null ? 1 : units)} units at R{(selectedPrice == null ? selectedPrice = price : selectedPrice).toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn bg-green-600" on:click={() => place_sell_order(false)}>Continue</button>
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <!--<button class="btn btn-accent font-light" onclick="my_modal_4.showModal()">Sell at Market Price</button>
          <dialog id="my_modal_4" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Sell Order</h3>
              <p class="py-4">Please confirm your sell order for {units} units at R{price.toFixed(2)} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn bg-green-600" on:click={() => place_sell_order(true)}>Continue</button>
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>-->

          <!-- <button class="btn btn-success" onclick="my_modal_1.showModal()">Buy at market price</button> -->
        </div>

      </div>

    </div>
  </div>
</main>



<style>
</style>
