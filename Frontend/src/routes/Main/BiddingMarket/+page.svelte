<script>
  import { onMount } from "svelte";
  let interval;
  let ads;

  onMount(async () => {
    await fetchAds();
  });

  async function fetchAds() {
    try {
      const response = await fetch("http://localhost:8001/get_ads", 
        {
          method: "POST", 
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            "num_advertisements": 4
          })
        },
      );

      // console.log(response);

      ads = await response.json();
      console.log(ads);
    } catch (error) {
      console.log("Unable to fetch advertisements." + error)
    }
  }

  async function buyFunction() {
    
  }

  async function sellFunction() {
    try {
      const response = await fetch("http://localhost:8001/advertise",
        {
          method: "POST", 
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            "email": "email@example.com",
            "units": 75.0,
            "price": 75.0
          })
        },
      );

      console.log(await response.json());
    } catch (error) {
      console.log("Unable to send advertisement.\n" + error)
    }
  }
</script>

<main class="container mx-auto">
  <h1 class="text-2xl font-bold mb-4">Bidding Market</h1>
  <div class="overflow-x-auto">

    <div class="card card-compact w-96 bg-base-200 shadow-xl">
      <!-- <figure><img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" alt="Shoes" /></figure> -->
      <div class="card-body">
        <h2 class="card-title">Advertise Here</h2>
        <p>Contribute to the open market.</p>
        <div class="card-actions justify-end">
          <button on:click={sellFunction} class="btn btn-primary">Advertise</button>
        </div>
      </div>
    </div>

    {#if ads.length > 0}
      {#each ads as ad}
      <!-- this is not going to display anything until it reads something from the array -->
      <!-- remember to change the photo after that starts working -->
      <div class="card card-compact w-96 bg-base-200 shadow-xl">
        <!-- <figure><img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" alt="Shoes" /></figure> -->
        <div class="card-body">
          <h2 class="card-title">Sample Electricity Company</h2>
          <p>Seller information goes here</p>
          <div class="card-actions justify-end">
            <button on:click={buyFunction} class="btn btn-primary">Buy Now</button>
          </div>
        </div>
      </div>
      {/each}
    {/if}

    <div class="absolute bottom-0 left-24 mb-32">
      <!-- <button on:click={bidFunction} class="btn bg-green-600 w-28 h-14">Bid</button> -->


      <!-- <button on:click={sellFunction} class="btn bg btn-secondary btn-lg btn-wide text-slate-900">Sell</button> -->
    </div>
  </div>
</main>



<style>
</style>
