<script>
  import { onMount } from "svelte";
  let units, price;
  let ads = [];

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
            "num_advertisements": 5
          })
        },
      );

      // console.log(response);

      ads = await response.json();
      ads = ads.advertisements;
      console.log(ads);
    } catch (error) {
      console.log("Unable to fetch advertisements." + error)
    }
  }

  async function buyFunction(ad_id) {
    // console.log("Buying from advertisement id: " + ad_id);
    try {
      const response = await fetch("http://localhost:8001/purchase",
        {
          method: "POST", 
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            "ad_id": ad_id,
            "email": "email@example.com",
            "units": 1
          })
        },
      );
    } catch (error) {
      console.log("Unable to buy the selected amount.\n" + error)
    }
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
            "units": units,
            "price": price
          })
        },
      );
      let res;
      console.log(res = await response.json());

      if (res.status === "ok") {
        advertise_modal.close();
      } else {
        // Show that an error occured
      }
    } catch (error) {
      console.log("Unable to send advertisement.\n"/* + error */)
    }
  }
</script>

<main class="container mx-auto">
  <h1 class="text-2xl font-bold mb-4">Electricity Marketplace</h1>
  
  <!-- <button class="btn btn-secondary mb-3" onclick="help_modal.showModal()">Help</button> -->
 

  <div class="grid grid-flow-row grid-cols-3 gap-5 overflow-x-auto">

    <div class="card card-compact w-96 bg-slate-700 shadow-xl">
      <!-- <figure><img src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" alt="Shoes" /></figure> -->
      <div class="card-body">
        <h2 class="card-title">Advertise Here</h2>
        <p>Put your power up for sale and contribute to the open market!</p>
        <div class="card-actions justify-end">
          <button onclick="advertise_modal.showModal()" class="btn btn-primary">Advertise</button>
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
          <h2 class="card-title">Electricity Advertisement {ad.advertisement_id}</h2>
          <p>Offered Units: {ad.offered_units}</p>
          <p>Price: {ad.price}</p>
          <div class="card-actions justify-end">
            <button on:click={buyFunction(ad.advertisement_id)} class="btn btn-primary">Buy Now</button>
          </div>
        </div>
      </div>
      {/each}
    {/if}

    <div class="absolute bottom-6 max-w-full ">
      <!-- <button on:click={bidFunction} class="btn bg-green-600 w-28 h-14">Bid</button> -->
      <button on:click={fetchAds} class="btn bg btn-outline btn-primary btn-lg btn-wide text-slate-900">Refresh</button>
    </div>
  </div>

  <dialog id="advertise_modal" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Advertise</h3>
      <p class="py-4"></p>
      <div class="card">
        <form class="card-body" method="dialog">
          <!-- if there is a button in form, it will close the modal -->
          <div class="modal-middle">
            <div class="form-control">
              <input type="number" class="input input-bordered" placeholder="units" bind:value={units} required/>
            </div>
  
            <div class="form-control mt-4">
              <input type="number" class="input input-bordered" placeholder="price" bind:value={price} required/>
            </div>
          </div>

          <div class="modal-action">
            <div class="form-control">
              <button class="btn btn-primary" on:click={sellFunction}>Confirm</button>
            </div>
            
            <div class="form-control">
              <button class="btn" onclick="advertise_modal.close()">Cancel</button>
            </div>
          </div>
        </form>
      </div>
    </div>
  </dialog>
</main>



<style>
</style>
