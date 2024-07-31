<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  

  let data = {};
  let interval; 
  let advancedView = false; 
  let numDecimals = 2; 
  $: nodes = [1,2,3,4,5,6];

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
    document.getElementById("newNodeModal").showModal();
  }


</script>

<main class="container mx-auto w-full flex justify-center">

  <div class="max-w-min mx-20">
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

    <div class="card">
      <div class="card-title">Personal Information</div>
    </div>
  </div>

  <div class="min-w-max min-h-fit mx-10 flex-row">

    <div class="flex-col">
      <span class="text-3xl justify-start">
        Your Nodes
      </span>
      <span class="justify-end pl-96">
        <button class="btn btn-primary text-lg " on:click={createModal}>Add a Node</button>
      </span>
    </div>

    <dialog id="newNodeModal" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Add a Node</h3>
        <form class="">
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Name">
          </div>
        </form>
      </div>


      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
    

    {#each nodes as node}
      <div class="card card-side min-w-1/3 bg-base-300 my-2">
        <figure class="w-1/5 p-10">
          <img
            src="../src/images/house.png"
            alt="House node" />
        </figure>
        <div class="card-body">
          <h2 class="card-title">Node {node}</h2>
          <p class="">
            Node type, etc
          </p>
          <div class="card-actions justify-end">
            <button class="btn btn-ghost">Details</button>
          </div>
        </div>
      </div>

    {/each}


    
  </div>
  <div class="max-w-min mx-10">
    <div class="stats stats-vertical shadow"> 
      <div class="stat">
        <div class="stat-title">Total Comsumption</div>
        <div class="stat-value">1,024W</div>
      </div>
    
      <div class="stat">
        <div class="stat-title">Total Generation</div>
        <div class="stat-value">5W</div>
        
      </div>
    </div>

    <!-- node specific stats go here along with the transaction buttons -->
  </div>

</main>

