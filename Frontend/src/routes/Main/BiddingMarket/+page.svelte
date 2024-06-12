<script>
  import { onMount } from "svelte";
  import { uid } from "uid";
  let interval;
  

  // console.log(uid());
  let arrayofuids = [];
  var arrayofpricesbought = [];
  // const userid = uid();

  onMount(async () => {
    await checkMet();
    interval = setInterval(checkMet, 800);

    //return function runs when the component is unmounted
    return () => {
      clearInterval(interval);
    };
  });

  function getRandomPrice() {
    //for demo purposes
    let random = Math.random();
    let temp = random * (250.00 - 200.00) + 200.00;
    return temp.toFixed(2);
}


   async function removeelement(element) {
    for (let i = 0; i < arrayofuids.length; i++) {
      let tempy = arrayofuids[i];
      if (tempy == element) {
        arrayofuids.splice(i, 1);
      }
    }
  }

  export function removeelement2(currarray, element) {
    for (let i = 0; i < currarray.length; i++) {
      let tempy = currarray[i];
      if (tempy == element) {
        return currarray.splice(i, 1);
      }
    }
    return currarray.clone();
  };

  async function checkMet() {
    for (let i = 0; i < arrayofuids.length; i++) {
      const element = arrayofuids[i];
      const response = await fetch(`http://localhost:8001/met/${element}`);
      let text = await response.text();
      // console.log(text);
      if (text == "true") {
        console.log(`Bid met for agent with id of: ${element}`);
        removeelement(element);
      }
    }
    arrayofuids = arrayofuids;
  }

  async function bidFunction() {
    //https://api.coindesk.com/v1/bpi/currentprice.json
    let id = uid();
    arrayofuids.push(id);
    console.log(id);
    console.log(arrayofuids);
    const response = await fetch(`http://localhost:8001/bid/1118/100/${id}`); //have to insert endpoint
    let text = await response.text();
    console.log(text);

    // data = JSON.parse(text)
  }

  async function sellFunction() {
    //https://api.coindesk.com/v1/bpi/currentprice.json
    const response = await fetch("http://localhost:8001/sell/1500"); //have to insert endpoint
    let text = await response.text();
    console.log(text);
    // data = JSON.parse(text)
  }
</script>

<main class="container mx-auto">
  <h1 class="text-2xl font-bold mb-4">Bidding Market</h1>
  <div class="overflow-x-auto">
    
    <div class="card card-compact w-96 bg-base-100 shadow-xl">
      <figure><img src="$lib/assets/pylonBack.png" alt="Pylons (sample)" /></figure>
      <div class="card-body">
        <h2 class="card-title">Sample Electricity Company</h2>
        <p>Seller information goes here</p>
        <div class="card-actions justify-end">
          <button on:click={sellFunction} class="btn bg-green-600">View Offers</button>
        </div>
      </div>
    </div>

    
    {#each arrayofuids as id, i (id)}
    <!-- this is not going to display anything until it reads something from the array -->
    <div class="card card-compact w-96 bg-base-100 shadow-xl">
      <figure><img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" alt="Shoes" /></figure>
      <div class="card-body">
        <h2 class="card-title">Sample Electricity Company</h2>
        <p>Seller information goes here</p>
        <div class="card-actions justify-end">
          <button on:click={sellFunction} class="btn bg-green-600">Buy Now</button>
        </div>
      </div>
    </div>
    {/each}

    <div class="absolute bottom-0 left-24 mb-64">
      <button on:click={bidFunction} class="btn bg-green-600 w-28 h-14">Bid</button>
      <button on:click={sellFunction} class="btn bg bg-red-700 w-28 h-14 ml-2">Sell</button>
    </div>
  </div>
</main>



<style>
</style>
