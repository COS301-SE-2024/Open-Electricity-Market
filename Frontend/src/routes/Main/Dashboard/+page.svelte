<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  

  let data = {};
  let nodeName = '';
  let nodeLongitude = '';
  let nodeLatitude = '';
  $: nodes = [];

  onMount(async () => {
    await fetchStart();
    await fetchNodes();
  }); 

  async function fetchStart() {
    try {
      const response = await fetch("http://localhost:8000/start", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }
    });
    } catch(error){
      console.log("An error occurred sending a post to /start endpoint.");
    }
  };

  async function fetchNodes() {
    try {
      const response = await fetch("http://localhost:8001/get_nodes", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          // there's a chance it complains at you if you do this: 
          'Accept': 'application/json',
        },
        credentials: "include", 
        body: JSON.stringify({
          limit: 10
        })
      });
      // console.log("request being sent...");
      // console.log(response);
      
      const fdata = await response.json();
      
      nodes = fdata.data;
    } catch (error) {
      console.log("An error occurred while fetching nodes..\n", error);
    }
  };

  function createModal(){
    document.getElementById("newNodeModal").showModal();
  }

  async function createNode() {
    // only proceed if all fields filled in
    if (nodeName == '' || nodeLatitude == '' || nodeLongitude == '') {
      // maybe show an error
      return;
    }

    try {
      const response = await fetch("http://localhost:8001/add_node", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          // might also complain here, have Content-Type be your only header 
          // 'Accept': 'application/json',
        },
        credentials: "include", 
        body: JSON.stringify({
          name: nodeName, 
          location_x: Number(nodeLatitude), 
          location_y: Number(nodeLongitude)
        })
      });
      // console.log("request being sent...");
      // console.log(response);
      
      const fdata = await response.json();

      if (fdata.status === 'ok') {
        document.getElementById("newNodeModal").close();
        fetchNodes();
      }

    } catch (error) {
      console.log("An error occurred while creating a node..\n", error);
    }

    // submit the new node request and update the nodes dynamic nodes array
  }

</script>

<main class="container mx-auto w-full flex justify-center">

  <div class="max-w-min mx-20">
    <div class="stats stats-vertical"> 
      <div class="stat">
        <div class="stat-title">Available Credit</div>
        <div class="stat-value">R31,000</div>
      </div>
    
      <div class="stat">
        <div class="stat-title">Pending Transactions</div>
        <div class="stat-value">5</div>
      </div>
      
      <div class="stat">
        <div class="stat-title">Total Comsumption</div>
        <div class="stat-value">1,024W</div>
      </div>
    
      <div class="stat">
        <div class="stat-title">Total Generation</div>
        <div class="stat-value">5W</div>
      </div>
    </div>

    <div class="card">
      <div class="card-title">Personal Information</div>
    </div>
  </div>

  <div class="min-w-max min-h-fit mx-10 flex-row">

    <div class="flex-col">
      <span class="text-3xl justify-start pl-2">
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
            <input class="input input-bordered" type="text" placeholder="Name" bind:value={nodeName}>
          </div>
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Latitude" bind:value={nodeLatitude}>
          </div>
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Longtitude" bind:value={nodeLongitude}>
          </div>
          <div class="form-control mt-4">
            <button class="btn btn-primary" on:click={createNode}>Confirm</button>
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
          <h2 class="card-title">{node.name}</h2>
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

</main>

