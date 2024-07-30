<script>
import Chart from "$lib/Components/Chart.svelte";

let price = 0;
let units = 0;




</script>

<main class="container mx-auto p-4">
  <div class="flex flex-row">
    <div class="basis-2/3 border-2">
      <Chart></Chart>
    </div>
    <div class="basis-1/3 border-2">
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
            <button class="btn btn-success" >Buy</button>
            <button class="btn btn-error"  >Sell</button>
          </div>
        </form>

    </div>
  </div>
</main>



<style>
</style>
