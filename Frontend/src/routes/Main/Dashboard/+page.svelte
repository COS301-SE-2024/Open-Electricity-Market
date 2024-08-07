<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  import Cookies from 'js-cookie';
  import {goto} from "$app/navigation";
  import Map from '$lib/Components/MapDashboard.svelte';
  

  let data = {};
  let nodeName = '';
  let nodeLongitude = '';
  let nodeLatitude = '';

  $: nodeNameDetail = '';
  $: nodeLongitudeDetail = '';
  $: nodeLatitudeDetail = '';
  $: nodeToProduce = '';
  $: nodeToConsume = '';
  $: selectedNodeID = '';

  $: nodes = [];
  let amount; 
  let withdrawamount; 
  let totalamount = 0; 
  let firstname; 
  let lastname; 
  let email; 
  //open buy order variables
  // let orderid; 
  // let filledunits; 
  // let openbuyprice; 
  // let openbuyunits; 
  $: buyorders = [];
  //sell order variables
  // let orderidsell; 
  // let opensellprice; 
  // let offeredunits; 
  // let claimedunits; 
  $: sellorders = [];
  //variables for map input
   let latitude = '';
   let longtitude = '';

  onMount(async () => {
    await fetchStart();
    await fetchNodes();
    await getUserDetails();
    await listOpenBuys();
    await listOpenSells();
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

  async function fetchNodeDetails(node_id_in) {
    const response = await fetch("http://localhost:8001/node_details", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json', 
        'Accept': 'application/json'
      },
      credentials: "include", 
      body: JSON.stringify({
        node_id: node_id_in
      })
    });

    const fdata = await response.json();

    data = fdata.data;
    console.log(data);

    nodeNameDetail = data.name;
    nodeLatitudeDetail = data.location_x;
    nodeLongitudeDetail = data.location_y;
    nodeToProduce = data.units_to_produce;
    nodeToConsume = data.units_to_consume;
    selectedNodeID = data.node_id;
  }


  function createModal(){
    nodeName = nodeLatitude = nodeLongitude = '';
    document.getElementById("mapModal").showModal();
  }

  async function createNode() {
    // only proceed if all fields filled in
    if (nodeName == '' || latitude == '' || longtitude == '') {
      // maybe show an error
      return;
    }

    try {
      const response = await fetch("http://localhost:8001/add_node", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
        credentials: "include", 
        body: JSON.stringify({
          name: nodeName, 
          location_x: Number(latitude), 
          location_y: Number(longtitude)
        })
      });
      // console.log("request being sent...");
      // console.log(response);
      
      const fdata = await response.json();
      console.log(fdata);

      if (fdata.status === 'ok') {
        document.getElementById("mapModal").close();
        fetchNodes();
      }

    } catch (error) {
      console.log("An error occurred while creating a node..\n", error);
    }

    // submit the new node request and update the nodes dynamic nodes array
  }

  async function removeNode(nodeID) {
    // console.log("removing node: " + nodeID);
    document.getElementById("removeNodeConfirmation").close();
    const response = await fetch("http://localhost:8001/remove_node", {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      credentials: "include",
      body: JSON.stringify ({
        node_id: nodeID,
      })
    })

    const fdata = await response.json();

    if (fdata.message === "Node successfully removed") {
      // show the user something happened
      fetchNodes();
      nodeNameDetail = '';
    } else {
      // show the user something went wrong
    }
  }

  async function addFunds(){

    
    if(!amount){
      console.log("No amount was given.");
      return; 
    }

    console.log("Add funds function was called " + amount);
    try {
      const response = await fetch("http://localhost:8001/add_funds", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        body: JSON.stringify({
            funds: amount
          }),
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from add funds endpoint is this: ", data);
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the add funds endpoint:", error);
    }

    //if funds added then show confirmation modal
    if(data.message == 'Funds added'){
      document.getElementById("addfundsconfirmation").showModal();
      // amount = '';
      totalamount += amount; 
    } else {
      document.getElementById("addfundsrejection").showModal();
    }
  }


  async function withdrawFunds(){

    
    if(!withdrawamount){
      console.log("No amount was given.");
      return; 
    }

    try {
      const response = await fetch("http://localhost:8001/remove_funds", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        body: JSON.stringify({
            funds: withdrawamount
          }),
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from withdraw funds endpoint is this: ", data);
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the withdrawfunds:", error);
    }

    //if funds added then show confirmation modal
    if(data.message == 'Funds removed'){
      document.getElementById("removefundsconfirmation").showModal();
      // withdrawamount = '';
      totalamount -= withdrawamount; 
    } else {
      document.getElementById("removefundsrejection").showModal();
    }
  }



  async function getUserDetails(){

    
    try {
      const response = await fetch("http://localhost:8001/user_details", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from user details is: ", data);
      
    } catch (error) {
      console.log("There was an error fetching user details:", error);
    }

    if(data.message == "User details successfully retrieved"){
      totalamount = data.data.credit; 
      email = data.data.email; 
      firstname = data.data.first_name; 
      lastname = data.data.last_name;
    } else {
      // this is intended to reroute the user to the login page if they send an invalid session id
      sessionStorage.clear();
      goto("/login");
    }
  }

  function nullifyValues(){
    withdrawamount = '';
    amount = '';
  }


  async function listOpenBuys(){

     try {
      const response = await fetch("http://localhost:8001/list_open_buys", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from user details is: ", data);
      
    } catch (error) {
      console.log("There was an error fetching user details:", error);
    }

    if(data.message == "Successfully retrieved open buy orders"){
      // orderid = data.data.order_id; 
      // filledunits = data.data.filled_units; 
      // openbuyprice = data.data.price; 
      // openbuyunits = data.data.sought_units;
      buyorders = data.data;  
    }

  }

  async function listOpenSells(){

    try {
      const response = await fetch("http://localhost:8001/list_open_sells", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      console.log("Data received from user details is: ", data);
      
    } catch (error) {
      console.log("There was an error fetching user details:", error);
    }

    if(data.message == "Successfully retrieved open sell orders"){
      // orderidsell = data.data.order_id; 
      // opensellprice = data.data.price; 
      // offeredunits = data.data.offered_units; 
      // claimedunits = data.data.claimed_units; 
      sellorders = data.data; 
    }
  }

  function handleMapClick(lat, lng){
    latitude = lat; 
    longtitude = lng; 
    console.log("Marker position updated: " + lat + " " + lng);
  }


</script>

<main class="container mx-auto w-full sm:flex justify-center">

  <div class="min-w-1/6">
    
    <!-- change funds modals -->

    <dialog id = "add_modal" class="modal">
      <div class="modal-box">
        <h3 class="text-lg font-bold">Add funds</h3>
        <p class="py-4">Please enter an amount you would like to add.</p>
        <div class="form-control mt-4">
          <input class="input input-bordered" type="number" placeholder="Amount" required bind:value={amount}>
        </div>
      
        <div class="modal-action">
          <form method="dialog">
            <button class="btn bg-green-600" on:click="{addFunds}">Continue</button>
            <button class="btn bg-red-600">Cancel</button>
          </form>
        </div>
      </div>
    </dialog>


    <dialog id="remove_modal" class="modal">
      <div class="modal-box">
        <h3 class="text-lg font-bold">Withdraw funds</h3>
        <p>Please ente ran amount you would like to withdraw.</p>
          <div class="form-control mt-4">
              <input class="input input-bordered" type="number" placeholder="Amount" required bind:value={withdrawamount}>
            </div>
        <div class="modal-action">
          <form method="dialog">
            <button class="btn bg-green-600" on:click={withdrawFunds}>Continue</button>
            <button class="btn bg-red-500">Cancel</button>
          </form>
        </div>
      </div>
    </dialog>

    <div class="stats stats-vertical"> 
      <div class="stat">
        <div class="stat-title">Available Credit</div>
        <div class="stat-value">R{totalamount}</div>
      </div>

      <div class="flex-col min-w-max">
        <button class="btn btn-success mx-2 w-48" onclick="add_modal.showModal()">Add funds</button>
        <button class="btn btn-error mx-2 w-48" onclick="remove_modal.showModal()">Withdraw funds</button>
      </div>
      
      <h1 class="stat text-lg">
        Personal Information:
      </h1>
  
      <div class="stat">
          <div class="stat-title">Firstname</div>
          <div class="stat-value">{firstname}</div>
      </div>
  
      <div class="stat">
          <div class="stat-title">Lastname</div>
          <div class="stat-value">{lastname}</div>
      </div>
  
      <div class="stat">
          <div class="stat-title">Email</div>
          <div class="stat-value">{email}</div>
      </div>
    </div>
  </div>

  <div class="min-w-max min-h-fit mx-4 flex-row">

    <div class="flex-col">
      <span class="text-3xl justify-start pl-2">
        Your Nodes
      </span>
    </div>

    <!-- new node modals -->
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

    
    <dialog id="mapModal" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Add a Node</h3>
        <form class="">
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Name" bind:value={nodeName}>
          </div>
          <!-- <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Latitude" bind:value={nodeLatitude}>
          </div>
          <div class="form-control mt-4">
            <input class="input input-bordered" type="text" placeholder="Longtitude" bind:value={nodeLongitude}>
          </div> -->
          <div class="form-control mt-4">
            <Map onMapClick = {handleMapClick} />
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
          <div class="card-actions justify-end">
            <button class="btn btn-ghost" on:click={() => {fetchNodeDetails(node.node_id)}}>Details</button>
          </div>
        </div>
      </div>
    {/each}

    <div class="card card-side min-w-1/3 bg-base-300 my-2">
      <div class="card-body">
        <button class="btn btn-outline" on:click={createModal}>Add a New Node</button>
      </div>
    </div>
  </div>

  <div class="min-w-1/6">
    {#if nodeNameDetail != ''}
      <div class="stats stats-vertical"> 
        <div class="stat">
          <div class="stat-title">Node</div>
          <div class="stat-value">{nodeNameDetail}</div>
        </div>
      
        <div class="stat">
          <div class="stat-title">Node Location</div>
          <div class="stat-value">
            {nodeLongitudeDetail < 0 ? nodeLongitudeDetail.toFixed(3) * -1 + "S " : nodeLongitudeDetail.toFixed(3) + "N "} 
            {nodeLatitudeDetail < 0 ? nodeLatitudeDetail.toFixed(3) * -1 + "W": nodeLatitudeDetail.toFixed(3) + "E"}
          </div>
        </div>
        
        <div class="stat">
          <div class="stat-title">Available Comsumption</div>
          <div class="stat-value">{nodeToConsume}Wh</div>
        </div>

        <div class="stat">
          <div class="stat-title">Pending Generation</div>
          <div class="stat-value">{nodeToProduce}Wh</div>
        </div>

        <div class="flex-col min-w-max">
          <button class="btn btn-primary mx-2 w-48" on:click={() => {
              sessionStorage.setItem("node_id", selectedNodeID);
              //reroute to market 
              goto('../Main/BiddingMarket');
            }}>Transact with this node</button>
          <button class="btn btn-error mx-2 w-48" on:click={() => {
              document.getElementById("removeNodeConfirmation").showModal();
            }}>Remove this node</button>
        </div>
      </div>
    {/if}
    
    {#each buyorders as buyorder}
      <div class="card card-side min-w-1/3 bg-base-200 my-2">
        <div class="card-body">
          <h2 class="card-title">Buy order</h2>
          <p>
            Filled units: {buyorder.filled_units.toFixed(1)}<br>
            Max price: {buyorder.max_price}<br>
            Min price: {buyorder.min_price}<br>
            Wh: {buyorder.sought_units.toFixed(1)}<br>
          </p>
          <div class="card-actions ">
            
          </div>
        </div>
      </div>
    {/each}

    {#each sellorders as sellorder}
      <div class="card card-side min-w-1/3 bg-base-200 my-2">
        <div class="card-body">
          <h2 class="card-title">Sell order</h2>
          <p>
            Offered Units: {sellorder.offered_units}<br>
            Claimed Units: {sellorder.claimed_units}<br>
            Price: {sellorder.price.toFixed(2)}<br>
          </p>
          <div class="card-actions">
            
          </div>
        </div>
      </div>
    {/each}
  </div>

  <!-- confirm remove node modal -->

  <dialog id="removeNodeConfirmation" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Confirmation</h3>
      <p>Are you sure you want to permanently delete this node from your account?</p>
      <div class="modal-action">
        <button class="btn btn-error" on:click={removeNode(selectedNodeID)}>Yes</button>
        <button class="btn btn-outline" on:click={() => {
            document.getElementById("removeNodeConfirmation").close()
          }}>No</button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <!-- confirm change funds modals -->

  <dialog id="addfundsconfirmation" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">You have successfully added funds!</h3>
      <p>You have successfully added R{amount} to your account.</p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>


  <dialog id="removefundsconfirmation" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Withdrawal of funds successful!</h3>
      <p>You have successfully withdrew R{withdrawamount} from your account.</p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button on:click={nullifyValues}>close</button>
      </form>
    </dialog>

    <dialog id="removefundsrejection" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Withdrawal of funds was unsuccessful.</h3>
      <p>Withdrawal of R{withdrawamount} was unsuccessful. Please check your balance.</p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button on:click={nullifyValues}>close</button>
      </form>
    </dialog>

    <dialog id="addfundsrejection" class="modal">  
      <div class="modal-box">
        <h3 class="font-bold text-lg ">Addition of funds unsuccessful.</h3>
      <p>Addition of R{amount} was unsuccessful. Please enter a valid value.</p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button on:click={nullifyValues}>close</button>
      </form>
    </dialog>

</main>
