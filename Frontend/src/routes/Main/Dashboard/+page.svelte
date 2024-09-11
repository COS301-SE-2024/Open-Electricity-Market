<script>
  import { onMount } from "svelte";
  import {goto} from "$app/navigation";
  import Map from '$lib/Components/MapDashboard.svelte';
  import { API_URL_GRID, API_URL_MARKET, API_URL_AGENT } from '$lib/config.js';
  import Scroller from '@sveltejs/svelte-scroller';
  

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
  $: totalamount = null; // this is "Available credit"
  $: firstname = null;
  $: lastname = null;
  $: email = null;
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
   let appliance = ''; 
   let appliances = [
        'Washing Machine', 'Router', 'Vacuum', 'Dishwasher', 'Boiler', 'Hair Purifier',
        'Sound System', 'Coffee Machine', 'Phone Charger', 'Fridge', 'Radiator',
        'Dehumidifier', 'Microwave Oven', 'Laptop', 'Tv', 'Screen',
        'Fan', 'Air Conditioner', 'Computer', 'Printer', 'Dryer', 'Freezer'
    ];

  let generator = ''; 
  let category = ''; 
  let generators = [
        { type: "SolarPanel", category: "Home" },
        { type: "SolarPanel", category: "Industrial" },
        { type: "WindTurbine", category: "Small" },
        { type: "WindTurbine", category: "Medium" },
        { type: "WindTurbine", category: "Large" },
        { type: "NuclearReactor", category: "PWR" },
        { type: "NuclearReactor", category: "BWR" },
        { type: "NuclearReactor", category: "AGR" },
        { type: "NuclearReactor", category: "FNR" },
        { type: "NuclearReactor", category: "PHWR" },
        { type: "NuclearReactor", category: "HTGR" },
        { type: "NuclearReactor", category: "LWGR" },
        { type: "DieselGenerator", category: "Home" },
        { type: "DieselGenerator", category: "Industrial" },
        { type: "PetrolGenerator", category: "Home" },
        { type: "PetrolGenerator", category: "Industrial" },
        { type: "CoalGenerator", category: "Small" },
        { type: "CoalGenerator", category: "Medium" },
        { type: "CoalGenerator", category: "Large" },
        { type: "HydraulicTurbine", category: "Small" },
        { type: "HydraulicTurbine", category: "Medium" },
        { type: "HydraulicTurbine", category: "Large" }
  ]; 

  let uniqueGens = [...new Set(generators.map(generator=> generator.type))]; 


  onMount(async () => {
    await fetchStart();
    await fetchNodes();
    await getUserDetails();
    await listOpenBuys();
    await listOpenSells();

    // const buyOrderInterval = setInterval(listOpenBuys, 10000);
    // const sellOrderInterval = setInterval(listOpenSells, 10000);

    // return () => {
    //   clearInterval(buyOrderInterval);
    //   clearInterval(sellOrderInterval);
    // }
  }); 

  async function fetchStart() {
    try {
      const response = await fetch(`${API_URL_GRID}/start`, {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
      }
    });
    } catch(error){
      console.log("An error occurred sending a post to /start endpoint.");
    }
  };

  async function fetchNodes() {
    try {
      const response = await fetch(`${API_URL_MARKET}/get_nodes`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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
    const response = await fetch(`${API_URL_MARKET}/node_details`, {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json', 
        'Accept': 'application/json',
        'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
      },
      credentials: "include", 
      body: JSON.stringify({
        node_id: node_id_in
      })
    });

    const fdata = await response.json();

    if (fdata.error) {
      // console.log(fdata.error.code);
      if (fdata.error.code = '403') {
        goto('/login');
      }
    } else {
      data = fdata.data;
      // console.log(data);
  
      nodeNameDetail = data.name;
      nodeLatitudeDetail = data.location_x;
      nodeLongitudeDetail = data.location_y;
      nodeToProduce = data.units_to_produce;
      nodeToConsume = data.units_to_consume;
      selectedNodeID = data.node_id;
    }

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
      const response = await fetch(`${API_URL_MARKET}/add_node`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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
    const response = await fetch(`${API_URL_MARKET}/remove_node`, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
        'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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
      const response = await fetch(`${API_URL_MARKET}/add_funds`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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
      const response = await fetch(`${API_URL_MARKET}/remove_funds`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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
      const response = await fetch(`${API_URL_MARKET}/user_details`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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
      window.location.replace("/login");
    }
  }

  function nullifyValues(){
    withdrawamount = '';
    amount = '';
  }


  async function listOpenBuys(){

     try {
      const response = await fetch(`${API_URL_MARKET}/list_open_buys`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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
      const response = await fetch(`${API_URL_MARKET}/list_open_sells`, {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Bearer ${sessionStorage.getItem("Token")}`
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

  function formatCurrency(value) {
    value *= 100;
    value = Math.floor(value);
    value /= 100;

    value = Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'ZAR'
    }).format(value);

    return value.slice(2, value.length);
  }

  async function addAppliance(){
    let details = {
      "email": email,
      "node_id": selectedNodeID,
      "appliances": []
    };

    let onPeriods = [{
      "start": 15.0, 
      "end": 800.0,
    }];
    if(appliance){
      let applianceDetails = {
        "appliance_type": appliance.replace(/\s/g,''),
        "on_periods": onPeriods 
      };
      details.appliances.push(applianceDetails);
      try {
        const response = await fetch(`${API_URL_AGENT}/add_appliances`, {
          method: "POST",
          body : JSON.stringify(details), 
          headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json',
          },
          credentials: "include",
        });
        const fdata = await response.json();
        data = fdata;
        console.log("Data received from user details is: ", data);
      } catch (error) {
        console.log("There was an error with the add appliance endpoint: ", error); 
      }
    }
    else{
      console.log("Appliance was not selected."); 
    }
  }

  async function addGenerator(){

     let details2 = {
    "email": email, 
    "node_id":selectedNodeID,
    "generators": [] ,
    }

    let onPeriods = {
     
      "start": 15.0, 
      "end": 800.0, 
      
    }

    if(generator && category){
      console.log(generator + " "+ category); 
      let generatorDetails = {
        "generator_type": {[generator]: category}, 
        "on_periods": [onPeriods]
      }
      details2.generators.push(generatorDetails);
      //details2.generators.generator_type.push(onPeriods); 
      console.log(details2);
      try {
        const response = await fetch(`${API_URL_AGENT}/add_generators`,{
        method: "POST", 
        body: JSON.stringify(details2), 
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
        credentials: "include",
      });
      const fdata = await response.json(); 
      data = fdata; 
      console.log("Data received from add gen endpoint: ", data); 
     } catch (error) {
        console.log("There was an error with the add generator endpoint: ", error); 
      }
    }

  }

</script>

<main class="container sm:mx-auto w-full h-screen sm:flex justify-center">
  <!--first-->
  <div class="sm:w-1/3 h-screen flex flex-col">
    
    <!--Personal Info-->
    <span class="text-3xl text-white font-thin justify-start pl-2">
      Personal Information
    </span>
    
    <div class="h-1/2 stats stats-vertical w-full"> 
      <div class="stat">
        <div class="stat-title">Available Credit</div>
        {#if totalamount == null}
        <span class="loading loading-spinner loading-lg"></span>
        {:else}
        <div class="stat-value font-normal">{formatCurrency(totalamount)}</div>
        {/if}
      </div>

      <div class="stat flex min-w-max py-0 justify-center">
        <button class="btn btn-primary w-6/12" onclick="add_modal.showModal()">Add funds</button>
        <button class="btn btn-accent w-6/12" onclick="remove_modal.showModal()">Withdraw funds</button>
      </div>
  
      <div class="stat">
          <div class="stat-title  ">Firstname</div>
          {#if firstname == null}
            <span class="loading loading-spinner loading-lg"></span>
          {:else}
            <div class="stat-value font-light">{firstname}</div>
          {/if}
      </div>
  
      <div class="stat">
          <div class="stat-title">Lastname</div>
          {#if lastname == null}
            <span class="loading loading-spinner loading-lg"></span>
          {:else}
            <div class="stat-value font-light">{lastname}</div>
          {/if}
      </div>
  
      <div class="stat">
          <div class="stat-title">Email</div> 
          {#if email == null}
            <span class="loading loading-spinner loading-lg"></span>
          {:else}
            <div class="stat-value font-light">{email}</div>
          {/if}
      </div>
    </div>

    <!--Buy orders-->
    <div class = "card h-1/3 bg-base-100 mt-2">
      <span class="text-3xl text-white font-thin justify-start pl-2">
          Buy Orders
      </span>

      <div class=" h-60 overflow-auto">
        
        {#each buyorders as buyorder}
          <div class="card min-w-1/3 bg-base-100 my-2 border">
            <div class="card-body">
              <h2 class="card-title">Buy order</h2>
              <p>
                Filled units: {buyorder.filled_units.toFixed(1) + "Wh"}<br>
                Max price: {formatCurrency(buyorder.max_price)}<br>
                Min price: {formatCurrency(buyorder.min_price)}<br>
                Units bought: {Intl.NumberFormat().format(buyorder.sought_units) + "Wh"}<br>
              </p>
              <div class="card-actions ">
                <progress class="progress progress-primary" value="{buyorder.filled_units}" max="{buyorder.sought_units}"></progress>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>
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
        <p>Please enter an amount you would like to withdraw.</p>
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
  </div>

  <!--second-->
  <div class="sm:w-1/3 h-screen mx-4 flex flex-col">

    <!--Nodes-->
    <span class="basis text-3xl text-white font-thin justify-start pl-2">
      Your Nodes
    </span>
    <div class="h-1/2 flex flex-col">
      
      <div class = "card bg-base-100 overflow-y-auto">
        {#each nodes as node}
        {#if node.name == nodeNameDetail}
          <div class="card card-side border-4 border-primary min-w-1/3 bg-base-100 mb-2">
            <figure class="w-1/5 p-10">
              <img
                src="../src/images/house.png"
                alt="House node" />
            </figure>
            <div class="card-body">
              <h2 class="card-title font-light text-3xl">{node.name}</h2>
              <div class="card-actions justify-end">
                <button class="btn btn-primary" on:click={() => {fetchNodeDetails(node.node_id)}}>Details</button>
              </div>
            </div>
          </div>  
        {:else}
          <div class="card card-side border-4 border-white min-w-1/3 bg-base-100 mb-2">
            <figure class="w-1/5 p-10">
              <img
                src="../src/images/house.png"
                alt="House node" />
            </figure>
            <div class="card-body">
              <h2 class="card-title font-light text-3xl">{node.name}</h2>
              <div class="card-actions justify-end">
                <button class="btn btn-primary" on:click={() => {fetchNodeDetails(node.node_id)}}>Details</button>
              </div>
            </div>
          </div>
        {/if}
        {/each}
      </div>

      <!--Add New node-->
      <div class="card card-side min-w-1/3 bg-base-100 mt-2">
        <div class="card-body">
          <button class="btn btn-outline" on:click={createModal}>Add a New Node</button>
        </div>
      </div>
    </div>
    
    <!--Sell orders-->
    <div class = "card h-1/3 bg-base-100 mt-2">
      <span class="text-3xl text-white font-thin justify-start pl-2">
          Sell Orders
      </span>

      <div class = "h-60 overflow-y-auto">
        {#each sellorders as sellorder}
        <div class="card card-side min-w-1/3 bg-base-100 my-2 border">
          <div class="card-body">
            <h2 class="card-title">Sell order</h2>
            <p>
              Claimed Units: {sellorder.claimed_units.toFixed(1) + "Wh"}<br>
              Offered Units: {sellorder.offered_units.toFixed(1) + "Wh"}<br>
              Max price: {formatCurrency(sellorder.max_price)}<br>
              Min price: {formatCurrency(sellorder.min_price)}<br>
            </p>
            <div class="card-actions">
              <progress class="progress progress-accent" value="{sellorder.claimed_units}" max="{sellorder.offered_units}"></progress>
            </div>
          </div>
        </div>
        {/each}
      </div>
    </div>

    <!-- new node modals -->
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
  </div>

  <!--third-->
  <div class="sm:w-1/3">
    {#if nodeNameDetail != ''}
      <span class="text-3xl text-white font-thin justify-start pl-2">
        Node Details
      </span>
      <div class = "h-5/6 overflow-y-auto">
        <div class="stats stats-vertical w-full"> 
          <div class="stat">
            <div class="stat-title">Node</div>
            <div class="stat-value font-light">{nodeNameDetail}</div>
          </div>
          <!-- flex min-w-max py-0 justify-center -->
          <div class="stat flex w-full py-0 justify-center">
            <button class="btn btn-primary w-6/12" on:click={() => {
              sessionStorage.setItem("node_id", selectedNodeID);
              sessionStorage.setItem("node_name", nodeNameDetail);
              //reroute to market 
              goto('../Main/BiddingMarket');
            }}>Transact with this node</button>
            <button class="btn btn-error w-6/12" on:click={() => {
              document.getElementById("removeNodeConfirmation").showModal();
            }}>Remove node</button>
          </div>
          
          <div class="stat">
            <div class="stat-title">Node Location</div>
            <div class="stat-value font-light">
              {nodeLongitudeDetail < 0 ? nodeLongitudeDetail.toFixed(3) * -1 + "S " : nodeLongitudeDetail.toFixed(3) + "N "} 
              {nodeLatitudeDetail < 0 ? nodeLatitudeDetail.toFixed(3) * -1 + "W": nodeLatitudeDetail.toFixed(3) + "E"}
            </div>
          </div>
          
          <div class="stat">
            <div class="stat-title">Available Consumption</div>
            <div class="stat-value font-light">{Intl.NumberFormat().format(nodeToConsume)} Wh</div>
          </div>

          <div class="stat">
            <div class="stat-title">Pending Generation</div>
            <div class="stat-value font-light">{Intl.NumberFormat().format(nodeToProduce)} Wh</div>
          </div>

        </div>

        <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 my-2">
          <span class="text-3xl font-thin justify-start">
            Add an Appliance
          </span>

          <div class="form-control">
            <select bind:value={appliance} class="select select-bordered max-h-40 overflow-y-auto my-2">
                <option value="" disabled selected>Select an appliance</option>
                {#each appliances as appliance}
                    <option value={appliance}>{appliance}</option>
                {/each}
            </select>
            <button on:click={addAppliance} class="btn btn-primary my-2">Add Appliance</button>
          </div>
          <!-- selecting category  -->
          <div class = "form-control">
            <span class = "label">
              <span class = "label-text">Select a generator</span>
            </span>
            <select bind:value={generator} class="select select-bordered max-h-40 overflow-y-auto">
              <option value = "" disabled selected>Select a generator</option>
              {#each uniqueGens as type}
                    <option value={type}>{type}</option>
              {/each}
            </select>
            
            <select bind:value={category} class = "select select-bordered max-h-40 overflow-y-auto mt-4" disabled={!generator}>
              <option value = "" disabled selected>Select a category</option>
              {#each generators.filter(g=>g.type === generator) as {category}}
              <option value = {category}>{category}</option>
              {/each}
            </select>
            <button on:click={addGenerator} class="btn btn-primary mt-4">Add Generator</button>
          </div>
        </div>
      </div>
    {/if}
    
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
      <p>You have successfully added {formatCurrency(amount)} to your account.</p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

  <dialog id="removefundsconfirmation" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Withdrawal of funds successful!</h3>
      <p>You have successfully withdrew {formatCurrency(withdrawamount)} from your account.</p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

  <dialog id="removefundsrejection" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Withdrawal of funds was unsuccessful.</h3>
      <p>Withdrawal of {formatCurrency(withdrawamount)} was unsuccessful. Please check your balance.</p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

  <dialog id="addfundsrejection" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Addition of funds unsuccessful.</h3>
      <p>Addition of {formatCurrency(amount)} was unsuccessful. Please enter a valid value.</p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

</main>
