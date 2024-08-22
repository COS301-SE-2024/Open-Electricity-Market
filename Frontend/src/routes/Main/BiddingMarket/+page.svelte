<script>
import Chart from "$lib/Components/Chart.svelte";
import {onMount} from "svelte";
import { goto } from '$app/navigation';
import { API_URL_GRID, API_URL_MARKET } from '$lib/config.js';

let selectedPrice = 0;
$: price = 0;
let units = 1;

let selected_node_id = sessionStorage.getItem("node_id");
let selected_node_name = sessionStorage.getItem("node_name");

let data = {};

async function place_buy_order() {
  // TODO: add a check that fails if units <= 0

  let data = {
    "node_id": selected_node_id,
    "min_price": selectedPrice > 1 ? selectedPrice - 1 : 0.01,
    "max_price": selectedPrice + 1,
    "units": units
  }

  const response = await fetch(`${API_URL_MARKET}/buy_order`, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
    },
    body : JSON.stringify(data),
    credentials: 'include'
  });

  goto('../Main/Dashboard');
}

async function place_sell_order() {
  // TODO: add a check that fails if units <= 0

  let data = {
    "node_id": selected_node_id,
    "min_price": selectedPrice > 1 ? selectedPrice - 1 : 0.01,
    "max_price": selectedPrice + 1,
    "units": units
  }

  const response = await fetch(`${API_URL_MARKET}/sell_order`, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
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
        'Content-Type': 'application/json'
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
    <div class="md:basis-2/3 md:card md:border md:mr-5 md:p-4">
      <h1 class="md:text-5xl md:font-bold md:pt-8">Marketplace</h1>
      <Chart {data} class = "" />
    </div>
    <div class="md:basis-1/3 md:card md:border md:p-4 xs:pt-10">
      <span class="text-lg">Selected Node: </span> <br>
      <span class="text-3xl">{selected_node_name}</span> <br>
      <span class="text-lg">Current Average Market Price: </span> <br>
      <span class="text-3xl">R {price}</span> <br>
      
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
          <button class="btn btn-success" onclick="my_modal_1.showModal()">Buy</button>
          <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">Please confirm your buy order for {units} units at R{selectedPrice} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn bg-green-600" on:click={place_buy_order} >Continue</button>
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <button class="btn btn-success" onclick="my_modal_3.showModal()">Buy at Market Price</button>
          <dialog id="my_modal_3" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">Please confirm your buy order for {units} units at R{selectedPrice} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn bg-green-600" on:click={place_buy_order} >Continue</button>
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <button class="btn btn-error" onclick="my_modal_2.showModal()">Sell</button>
          <dialog id="my_modal_2" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Sell Order</h3>
              <p class="py-4">Please confirm your sell order for {units} units at R{selectedPrice} </p>
              <div class="modal-action">
                <form method="dialog">
                  <button class="btn bg-green-600" on:click={place_sell_order}>Continue</button>
                  <button class="btn bg-red-500">Cancel</button>
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
