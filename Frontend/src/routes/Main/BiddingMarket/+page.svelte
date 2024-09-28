<script>
  import Chart from "$lib/Components/Chart.svelte";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { API_URL_GRID, API_URL_MARKET } from "$lib/config.js";
  import { createChart } from "lightweight-charts";
  import PriceChartD3 from "$lib/Components/PriceChartD3.svelte";
  // ***********************************************

  // **********************************************
  let selectedPricePerkWh = 0;
  $: pricePerWh = 0;
  let units = 1;
  let chartPeriod = "Day1";

  let selected_node_id = sessionStorage.getItem("node_id");
  let selected_node_name = sessionStorage.getItem("node_name");

  let chartData = [];
  async function reset_price() {
    // console.log("setting price to " + price);
    selectedPricePerkWh = pricePerWh * 1000;
  }

  async function place_buy_order(at_market_price) {
    if (at_market_price == true) {
      selectedPricePerkWh = pricePerWh * 1000;
    }

    let data = {
      node_id: selected_node_id,
      price: selectedPricePerkWh / 1000,
      units: units * 1000,
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
    if (at_market_price == true) {
      selectedPricePerkWh = pricePerWh * 1000;
    }

    let data = {
      node_id: selected_node_id,
      price: selectedPricePerkWh / 1000,
      units: units * 1000,
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
    // token check and refresh
    const session = sessionStorage.getItem("Token");

    if (!session) {
      goto("/login");
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
        sessionStorage.setItem("Token", fdata.data.token);
      } else {
        goto("/login");
      }
    }

    await fetchPriceHistory(chartPeriod);
    await fetchData();
    // let interval = setInterval(fetchPriceHistory, 2000);

    selectedPricePerkWh = pricePerWh * 1000;

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

      let data = fdata.data;

      pricePerWh = data.price.toFixed(2);
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
      // console.log(fdata);

      chartData.prices = fdata.data.map((item) =>
        parseFloat((item.price * 1000).toFixed(2))
      );
      chartData.timestamps = fdata.data.map((item) => item.timestamp);
      chartData.chartPeriod = chartPeriod;
      // console.log("This is data for the chart: " + data);
    } catch (error) {
      console.log("An error occurred while fetching price history");
    }
  }
</script>

<main class="container mx-auto p-4">
  <div class="md:flex md:flex-row">
    <div class="md:basis-2/3 bg-base-100 card md:mr-5 md:mb-0 mb-4 p-6">
      <span class="flex">
        <h1 class="text-5xl font-light">Marketplace</h1>
        <div class="form-control mt-3 ml-auto">
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
      </span>
      <div class="w-full">
        <Chart {chartData} />
      </div>
      <!-- <PriceChartD3 id = "chartPrice" />  -->
    </div>
    <div class="md:basis-1/3 card bg-base-100 p-6 max-h-fit">
      <h1 class="text-4xl font-light">Node Info</h1>
      <hr />
      <br />
      <span class="text-lg font-light">Selected Node: </span>
      <span class="text-3xl">{selected_node_name}</span> <br />
      <hr />
      <br />
      <span class="text-lg font-light"
        >Current Average Market Price: (Rands per kWh)</span
      >
      <span class="text-3xl">R {(pricePerWh * 1000).toFixed(2)}</span> <br />
      <hr />
      <br />

      <form>
        <div class="form-control w-full">
          <label for="buy_price" class="font-light"> Price </label>
          <div class="flex max-w-full">
            <input
              id="buy_price"
              type="number"
              step="0.01"
              placeholder={selectedPricePerkWh}
              class="basis-2/3 input input-bordered font-bold"
              name="buy_price"
              required
              bind:value={selectedPricePerkWh}
            />
            <button
              class="btn btn-primary basis-1/3 ml-1"
              title="Resets price back to current average market price"
              on:click={reset_price}>Market price</button
            >
          </div>
        </div>
        <br />
        <hr />
        <br />
        <div class="form-control mt-1">
          <label for="amount" class="font-light"> Kilowatt-hours </label>
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
            class="basis-1/2 btn btn-primary mx-1"
            onclick="my_modal_1.showModal()">Buy</button
          >
          <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Buy Order</h3>
              <p class="py-4">
                Please confirm your buy order for {units == null || units <= 0
                  ? (units = 1)
                  : units} kWh at R{(selectedPricePerkWh == null
                  ? (selectedPricePerkWh = pricePerWh)
                  : selectedPricePerkWh
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
            class="basis-1/2 btn btn-accent mx-1"
            onclick="my_modal_2.showModal()">Sell</button
          >
          <dialog id="my_modal_2" class="modal">
            <div class="modal-box">
              <h3 class="text-lg font-bold">Confirm Sell Order</h3>
              <p class="py-4">
                Please confirm your sell order for {units == null || units <= 0
                  ? (units = 1)
                  : units} kWh at R{(selectedPricePerkWh == null
                  ? (selectedPricePerkWh = pricePerWh)
                  : selectedPricePerkWh
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
  input[type="number"]::-webkit-inner-spin-button,
  input[type="number"]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
</style>
