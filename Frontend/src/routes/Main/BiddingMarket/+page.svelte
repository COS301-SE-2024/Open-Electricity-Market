<script>
import Chart from "$lib/Components/Chart.svelte";
import {onMount} from "svelte";

let price = 0;
let units = 0;

let data = {};


onMount(async () => {
  fetchData();
  let interval = setInterval(fetchData, 1000);

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

<main class="container mx-auto p-4">
  <div class="flex flex-row">
    <div class="basis-2/3 border-2 mr-5">
      <h1 class="text-5xl font-bold pt-8">Market</h1>
      <Chart {data}  ></Chart>
    </div>
    <div class="basis-1/3 border-2 p-2">
        <form>
          <div class="form-control mt-1">
            <label for="buy_price"> Price </label>
            <input type="number" placeholder="5" class="input input-bordered" name="buy_price" required bind:value={price}/>
          </div>

          <div class="form-control mt-1">
            <label for="amount"> Number of units </label>
            <input type="number" placeholder="5" class="input input-bordered" name="amount" required bind:value={units}/>
          </div>

          <div class="mt-1">
            <button class="btn btn-success" onclick="my_modal_1.showModal()">Buy</button>
            <dialog id="my_modal_1" class="modal">
              <div class="modal-box">
                <h3 class="text-lg font-bold">Confirm Buy Order</h3>
                <p class="py-4">Please confirm your buy order for {units} units at R {price} </p>
                <div class="modal-action">
                  <form method="dialog">
                    <button class="btn">Continue</button>
                    <button class="btn">Cancel</button>
                  </form>
                </div>
              </div>
            </dialog>
            <button class="btn btn-error" onclick="my_modal_2.showModal()">Sell</button>
            <dialog id="my_modal_2" class="modal">
              <div class="modal-box">
                <h3 class="text-lg font-bold">Confirm Sell Order</h3>
                <p class="py-4">Please confirm your sell order for {units} units at R {price} </p>
                <div class="modal-action">
                  <form method="dialog">
                    <button class="btn">Continue</button>
                    <button class="btn">Cancel</button>
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
</style>
