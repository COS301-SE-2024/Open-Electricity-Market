<script>
  import Chart from "$lib/Components/Chart.svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { API_URL_GRID, API_URL_MARKET } from "$lib/config.js";
  import { createChart } from "lightweight-charts";
  import PriceChartD3 from "$lib/Components/PriceChartD3.svelte";
  // ***********************************************

  // **********************************************
  let selectedPrice = 0;
  $: price = 0;
  let units = 1;
  let chartPeriod = "Day1";

  let selected_node_id = sessionStorage.getItem("node_id");
  let selected_node_name = sessionStorage.getItem("node_name");

  let data = [];
  async function reset_price() {
    // console.log("setting price to " + price);
    selectedPrice = price;
  }

  async function place_buy_order(at_market_price) {
    // TODO: add a check that fails if units <= 0

    if (at_market_price == true) {
      selectedPrice = price;
    }

    let data = {
      node_id: selected_node_id,
      price: selectedPrice,
      units: units,
    };

    const response = await fetch(`${API_URL_MARKET}/buy_order`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
      },
      body: JSON.stringify(data),
      credentials: "include", 
    });

    goto("../Main/Dashboard");
  }

  async function place_sell_order(at_market_price) {
    // TODO: add a check that fails if units <= 0

    if (at_market_price == true) {
      selectedPrice = price;
    }

    let data = {
      node_id: selected_node_id,
      price: selectedPrice,
      units: units,
    };

    const response = await fetch(`${API_URL_MARKET}/sell_order`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
      },
      body: JSON.stringify(data),
      credentials: "include",
    });

    goto("../Main/Dashboard");
  }

  onMount(async () => {
    await fetchPriceHistory(chartPeriod);
    // let interval = setInterval(fetchPriceHistory, 2000);

    selectedPrice = price;

    //return function runs when the component is unmounted
    return () => {
      // clearInterval(interval);
    };
  });

  async function fetchData() {
    try {
      const response = await fetch(`${API_URL_MARKET}/price_view`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
      });

      const fdata = await response.json();

      data = fdata.data;
      // console.log(data)

      price = data.price;
    } catch (error) {
      console.log(
        "There was an error fetching the JSON for the overview..",
        error
      );
    }
  }

  async function fetchPriceHistory(numhours) {
    try {
      const response = await fetch(`${API_URL_MARKET}/price_history`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        credentials: "include",
        body: JSON.stringify({
          time_frame: numhours,
        }),
      });

      const fdata = await response.json();
      console.log(fdata);
      data = fdata.data.map((item) => parseFloat(item.price.toFixed(2)));
      price = data.length > 0 ? data[data.length - 1] : 0;
      console.log("This is data for the chart: " + data);
    } catch (error) {
      console.log("An error occurred while fetching price history", error);
    }
  }
</script>

<main class="container mx-auto p-4">
  <div class="md:flex md:flex-row">
    <div class="md:basis-2/3 bg-base-100 md:card md:mr-5 md:p-4">
      <h1 class="md:text-5xl md:font-light md:pt-8">Marketplace</h1>
      <div class="form-control mt-3">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <select
          bind:value={chartPeriod}
          class="select select-bordered max-h-40 overflow-y-auto"
          on:change={() => fetchPriceHistory(chartPeriod)}
        >
          <option value="Day1" default>24h</option>
          <option value="Week1">7d</option>
          <option value="Month1">1M</option>
          <option value="Month3">3M</option>
          <option value="Month6">6M</option>
          <option value="Year1">1Y</option>
          <!-- This works so long as the endpoint retrieves different num values for longer hours etc... to be discussed with ruan -->
        </select>
      </div>
      <Chart {data} />
      <!-- <PriceChartD3 id = "chartPrice" />  -->
    </div>
    <div class="md:basis-1/3 md:card bg-base-100 md:p-4 xs:pt-10">
      <h1 class="md:text-4xl md:font-light md:pt-4">Node Info</h1>
      <hr />
      <br />
      <span class="text-lg font-light">Selected Node: </span>
      <span class="text-3xl">{selected_node_name}</span> <br />
      <hr />
      <br />
      <span class="text-lg font-light">Current Average Market Price: </span>
      <span class="text-3xl">R {price.toFixed(2)}</span> <br />
      <hr />
      <br />

      <form>
        <div class="form-control mt-1">
          <label for="buy_price" class="font-light"> Price </label>
          <div class="flex">
            <input
              id="buy_price"
              type="number"
              step = "0.01"
              placeholder={selectedPrice}
              class="basis-2/3 input input-bordered font-bold"
              name="buy_price"
              required
              bind:value={selectedPrice}
            />
            <span class="md:p-1"> </span>
            <button
              class="btn btn-primary basis-1/3"
              title="Resets price back to current average market price"
              on:click={reset_price}>Market price</button
            >
          </div>
        </div>
        <br />
        <hr />
        <br />
        <div class="form-control mt-1">
          <label for="amount" class="font-light"> Watt-hours </label>
          <input
            id="buy_units"
            type="number"
            placeholder={units == null ? 1 : units}
            class="input input-bordered font-bold"
            name="amount"
            required
            bind:value={units}
          />
        </div>

        <div class="mt-1 xs:pt-5 flex justify-center">
          <button
            class="md:basis-1/2 btn btn-primary mx-1"
            onclick="my_modal_1.showModal()">Buy</button
          >
          <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">
                Please confirm your buy order for {units == null
                  ? (units = 1)
                  : units} units at R{(selectedPrice == null
                  ? (selectedPrice = price)
                  : selectedPrice
                ).toFixed(2)}
              </p>
              <div class="modal-action">
                <form method="dialog">
                  <button
                    class="btn bg-green-600"
                    on:click={() => place_buy_order(false)}>Continue</button
                  >
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>

          <button
            class="md:basis-1/2 btn btn-accent mx-1"
            onclick="my_modal_2.showModal()">Sell</button
          >
          <dialog id="my_modal_2" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Sell Order</h3>
              <p class="py-4">
                Please confirm your sell order for {units == null
                  ? (units = 1)
                  : units} units at R{(selectedPrice == null
                  ? (selectedPrice = price)
                  : selectedPrice
                ).toFixed(2)}
              </p>
              <div class="modal-action">
                <form method="dialog">
                  <button
                    class="btn bg-green-600"
                    on:click={() => place_sell_order(false)}>Continue</button
                  >
                  <button class="btn bg-red-500">Cancel</button>
                </form>
              </div>
            </div>
          </dialog>
        </div>
      </form>
    </div>
  </div>
</main>

<style>
   input[type="number"] {
  -moz-appearance: textfield;
}
input[type=number]::-webkit-inner-spin-button,
input[type=number]::-webkit-outer-spin-button{
  -webkit-appearance: none;
  margin: 0; 
}
</style>
