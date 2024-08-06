<script>
import Chart from "$lib/Components/Chart.svelte";
import {onMount} from "svelte";

let price = 0;
let units = 0;

let data = {};

let test_node_id = sessionStorage.getItem("node_id");

async function place_buy_order() {
  let data = {
    "node_id": test_node_id,
    // "price": price,
    "min_price": price >= 0.05 ? price - 0.05 : 0.01,
    "max_price": price + 0.05,
    "units": units
  }

  const response = await fetch("http://localhost:8001/buy_order", {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
    },
    body : JSON.stringify(data),
    credentials: 'include'
  });

  console.log(await response.json())

}

async function place_sell_order() {
  let data = {
    "node_id": test_node_id,
    // "price": price,
    "min_price": price >= 0.05 ? price - 0.05 : 0.01,
    "max_price": price + 0.05,
    "units": units
  }

  const response = await fetch("http://localhost:8001/sell_order", {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
    },
    body : JSON.stringify(data),
    credentials: 'include'
  });

  console.log(await response.json())

}

onMount(async () => {
  fetchData();
  let interval = setInterval(fetchData, 2000);

  //return function runs when the component is unmounted
  return() => {
    clearInterval(interval);
  };

  async function fetchData() {

    try {
      const response = await fetch("http://localhost:8001/price_view", {
        method: "POST",
        headers: {
          'Content-Type': 'application/json'
        }

      });


      // const response = fetch("http://localhost:8000");
      const fdata = await response.json();

      data = fdata.data
      console.log(data)

    } catch (error) {
      console.log("There was an error fetching the JSON for the overview..", error);
    }
  }


});


</script>

<main class="container mx-auto p-4 ">
  <div class="md:flex md:flex-row xs: xs: xs: ">
    <div class="md:basis-2/3 md:border md:mr-5 md:p-4 xs:">
      <h1 class="md:text-5xl md:font-bold md:pt-8">Market</h1>
      <Chart {data} class = "" />
    </div>
    <div class="md:basis-1/3 md:border md:p-4 xs:pt-10">
        <form>
          <div class="form-control mt-1">
            <label for="buy_price"> Price </label>
            <input type="number" placeholder="5" class="input input-bordered" name="buy_price" required bind:value={price}/>
          </div>

          <div class="form-control mt-1">
            <label for="amount"> Number of units </label>
            <input type="number" placeholder="5" class="input input-bordered" name="amount" required bind:value={units}/>
          </div>

          <div class="mt-1 xs:pt-5">
            <button class="btn btn-success" onclick="my_modal_1.showModal()">Buy</button>
            <dialog id="my_modal_1" class="modal">
              <div class="modal-box">
                <h3 class="text-lg font-bold">Confirm Buy Order</h3>
                <p class="py-4">Please confirm your buy order for {units} units at R{price} </p>
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
                <p class="py-4">Please confirm your sell order for {units} units at R{price}</p>
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
