<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  

  let data = {};
  let interval; 
  let advancedView = false; 
  let numDecimals = 2; 

  onMount(async () => {
    await fetchStart();
    await fetchData();
    interval = setInterval(fetchData, 10000);

    //return function runs when the component is unmounted 
    return() => {
      clearInterval(interval);
     
    };
  });

  async function fetchStart() {

      try {
        const response = await fetch("http://localhost:8000/start", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
  }
    catch(error){
      console.log("There was an error sending a post to /start endpoint.");
    }
  };
 async function fetchData() {

      try {
        const response = await fetch("http://localhost:8000/overview", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
        console.log("request being sent...");
        console.log(response);
        // const response = fetch("http://localhost:8000");
        const fdata = await response.json();
        console.log(data);
        //Voltage 1,2,3 as well as price
        data = fdata; 
      } catch (error) {
        console.log("There was an error fetching the JSON for the overview..", error);
      }
  };

  function createModal(){
    
  }


</script>

<main class="container mx-auto w-full flex">

  <div class="max-w-min">
    <div class="stats stats-vertical shadow"> 
      <div class="stat">
        <div class="stat-title">Available Credit</div>
        <div class="stat-value">R31,000</div>
      </div>
    
      <div class="stat">
        <div class="stat-title">Pending Transactions</div>
        <div class="stat-value">5</div>
        
      </div>
    </div>
  </div>

  <div class="w-96 min-h-fit ">
    <div class="card bg-neutral w-60 m-2">
    <figure class="px-10 pt-10">
      <img
        src="../src/images/add-node.png"
        alt="Shoes"
        class="rounded-xl" />
    </figure>
    <div class="card-body items-center text-center">
      <!-- <h2 class="card-title">New Node</h2> -->
        <div class="card-actions items-center text-center">
          <button class="btn btn-primary text-lg" on:click={createModal}>Add a Node</button>
        </div>
      </div>
    </div>

    <div class="card card-side w-full bg-base-300 shadow-x m-2">
      <figure>
        <img
          src="https://img.daisyui.com/images/stock/photo-1635805737707-575885ab0820.webp"
          alt="Movie" />
      </figure>
      <div class="card-body">
        <h2 class="card-title">Node 1</h2>
        <p>Current consumption: ...<br/>
        Other stats: ...
        </p>
        <div class="card-actions justify-end">
          <button class="btn btn-primary">Transact</button>
        </div>
      </div>
    </div>

    
  </div>

</main>

