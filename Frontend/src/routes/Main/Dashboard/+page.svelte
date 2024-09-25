<script>
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import Map from "$lib/Components/MapDashboard.svelte";
  import { API_URL_GRID, API_URL_MARKET, API_URL_AGENT } from "$lib/config.js";

  let data = {};
  let nodeName = "";
  let nodeLatitude = "";
  let nodeLongitude = "";

  $: nodeNameDetail = "";
  $: nodeLatitudeDetail = "";
  $: nodeLongitudeDetail = "";
  $: nodeToProduce = "";
  $: nodeToConsume = "";
  $: selectedNodeID = "";

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
  let latitude = "";
  let longtitude = "";
  let appliance = "";
  let appliances = [
    "Washing Machine",
    "Router",
    "Vacuum",
    "Dishwasher",
    "Boiler",
    "Hair Purifier",
    "Sound System",
    "Coffee Machine",
    "Phone Charger",
    "Fridge",
    "Radiator",
    "Dehumidifier",
    "Microwave Oven",
    "Laptop",
    "Tv",
    "Screen",
    "Fan",
    "Air Conditioner",
    "Computer",
    "Printer",
    "Dryer",
    "Freezer",
  ];

  let generator = "";
  let category = "";
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
    { type: "HydraulicTurbine", category: "Large" },
  ];

  $: addWot = "";

  let uniqueGens = [...new Set(generators.map((generator) => generator.type))];

  //viewing appliances and generators
  $: applianceNames = new Set();
  $: generatorNames = [];

  onMount(async () => {
    // token check and refresh
    const session = sessionStorage.getItem("Token");
    
    if (!session) {
      goto("/login")
    } else {
      const response = await fetch(`${API_URL_MARKET}/token_refresh`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });

      const fdata = await response.json();

      // console.log(fdata);
      if (!fdata.error) {
        // swap out to the new token
        sessionStorage.removeItem("Token");
        sessionStorage.setItem("Token", fdata.data.token)
      } else {
        goto("/login")
      }
    }


    await fetchStart();
    await fetchNodes();
    await getUserDetails();
    await listOpenBuys();
    await listOpenSells();

    const buyOrderInterval = setInterval(listOpenBuys, 10000);
    const sellOrderInterval = setInterval(listOpenSells, 10000);

    return () => {
      clearInterval(buyOrderInterval);
      clearInterval(sellOrderInterval);
    };
  });

  async function fetchStart() {
    try {
      const response = await fetch(`${API_URL_GRID}/start`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
      });
    } catch (error) {
      console.log("An error occurred sending a post to /start endpoint.");
    }
  }

  async function fetchNodes() {
    try {
      const response = await fetch(`${API_URL_MARKET}/get_nodes`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
        body: JSON.stringify({
          limit: 10,
        }),
      });
      // console.log("request being sent...");
      // console.log(response);

      const fdata = await response.json();
      nodes = fdata.data;
      //console.log("Nodes retrieved: ",nodes)
      listOpenBuys();
      listOpenSells();
    } catch (error) {
      console.log("An error occurred while fetching nodes..\n", error);
    }
  }

  async function fetchNodeDetails(node_id_in) {
    //console.log(node_id_in)
    const response = await fetch(`${API_URL_MARKET}/node_details`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
        Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
      },
      credentials: "include",
      body: JSON.stringify({
        node_id: node_id_in,
      }),
    });

    const fdata = await response.json();

    if (fdata.error) {
      // console.log(fdata.error.code);
      if ((fdata.error.code = "403")) {
        goto("/login");
      }
    } else {
      data = fdata.data;
      //console.log(data);

      nodeNameDetail = data.name;
      nodeLatitudeDetail = data.location_y;
      nodeLongitudeDetail = data.location_x;
      nodeToProduce = data.units_to_produce;
      nodeToConsume = data.units_to_consume;
      //selectedNodeID = data.node_id;
      selectedNodeID = node_id_in;
      addWot = "a";
    }
  }

  function createModal() {
    nodeName = nodeLatitude = nodeLongitude = "";
    document.getElementById("mapModal").showModal();
  }

  async function createNode() {
    // only proceed if all fields filled in
    if (nodeName == "" || latitude == "" || longtitude == "") {
      // maybe show an error
      let errorToast;
      if (nodeName == "" && latitude != "") {
        errorToast = document.getElementById("errorNodeName");
      } else {
        errorToast = document.getElementById("errorToast");
      }
      errorToast.style.display = "block";
      setTimeout(() => {
        errorToast.style.display = "none";
      }, 3000);
      return;
    }

    try {
      const response = await fetch(`${API_URL_MARKET}/add_node`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
        body: JSON.stringify({
          name: nodeName,
          location_y: Number(latitude),
          location_x: Number(longtitude),
        }),
      });
      // console.log("request being sent...");
      // console.log(response);

      const fdata = await response.json();
      //console.log(fdata);

      if (fdata.status === "ok") {
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
        "Content-Type": "application/json",
        Accept: "application/json",
        Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
      },
      credentials: "include",
      body: JSON.stringify({
        node_id: nodeID,
      }),
    });

    const fdata = await response.json();

    if (fdata.message === "Node successfully removed") {
      // show the user something happened
      fetchNodes();
      nodeNameDetail = "";
    } else {
      // show the user something went wrong
    }
  }

  async function addFunds() {
    if (!amount) {
      console.log("No amount was given.");
      return;
    }

    console.log("Add funds function was called " + amount);
    try {
      const response = await fetch(`${API_URL_MARKET}/add_funds`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          funds: amount,
        }),
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      //console.log("Data received from add funds endpoint is this: ", data);
    } catch (error) {
      console.log(
        "There was an error fetching the JSON for the add funds endpoint:",
        error
      );
    }

    //if funds added then show confirmation modal
    if (data.message == "Funds added") {
      document.getElementById("addfundsconfirmation").showModal();
      // amount = '';
      totalamount += amount;
    } else {
      document.getElementById("addfundsrejection").showModal();
    }
  }

  async function withdrawFunds() {
    if (!withdrawamount) {
      console.log("No amount was given.");
      return;
    }

    try {
      const response = await fetch(`${API_URL_MARKET}/remove_funds`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          funds: withdrawamount,
        }),
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      //console.log("Data received from withdraw funds endpoint is this: ", data);
    } catch (error) {
      console.log(
        "There was an error fetching the JSON for the withdrawfunds:",
        error
      );
    }

    //if funds added then show confirmation modal
    if (data.message == "Funds removed") {
      document.getElementById("removefundsconfirmation").showModal();
      // withdrawamount = '';
      totalamount -= withdrawamount;
    } else {
      document.getElementById("removefundsrejection").showModal();
    }
  }

  async function getUserDetails() {
    try {
      const response = await fetch(`${API_URL_MARKET}/user_details`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      //console.log("Data received from user details is: ", data);
    } catch (error) {
      console.log("There was an error fetching user details:", error);
    }

    if (data.message == "User details successfully retrieved") {
      totalamount = data.data.credit;
      email = data.data.email;
      firstname = data.data.first_name;
      lastname = data.data.last_name;
      sessionStorage.setItem("email", email);
    } else {
      // this is intended to reroute the user to the login page if they send an invalid session id
      sessionStorage.clear();
      window.location.replace("/login");
    }
  }

  function nullifyValues() {
    withdrawamount = "";
    amount = "";
  }

  async function listOpenBuys() {
    console.log("fetching open buys")
    try {
      const response = await fetch(`${API_URL_MARKET}/list_open_buys`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      buyorders = [];
      //console.log("Data received from user details is: ", data);
    } catch (error) {
      console.log("There was an error fetching user details:", error);
    }

    if (data.message == "Successfully retrieved open buy orders") {
      // orderid = data.data.order_id;
      // filledunits = data.data.filled_units;
      // openbuyprice = data.data.price;
      // openbuyunits = data.data.sought_units;
      buyorders = data.data;
    }
  }

  async function listOpenSells() {
    try {
      const response = await fetch(`${API_URL_MARKET}/list_open_sells`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        credentials: "include",
      });
      const fdata = await response.json();
      data = fdata;
      sellorders = [];
      //console.log("Data received from user details is: ", data);
    } catch (error) {
      console.log("There was an error fetching user details:", error);
    }

    if (data.message == "Successfully retrieved open sell orders") {
      // orderidsell = data.data.order_id;
      // opensellprice = data.data.price;
      // offeredunits = data.data.offered_units;
      // claimedunits = data.data.claimed_units;
      sellorders = data.data;
    }
  }

  function handleMapClick(lat, lng) {
    latitude = lat;
    longtitude = lng;
    console.log("Marker position updated: " + lat + " " + lng);
  }

  function formatCurrency(value) {
    value *= 100;
    value = Math.floor(value);
    value /= 100;

    value = Intl.NumberFormat("en-US", {
      style: "currency",
      currency: "ZAR",
    }).format(value);

    return value.slice(2, value.length);
  }

  async function addAppliance() {
    let details = {
      email: email,
      node_id: selectedNodeID,
      appliances: [],
    };

    let onPeriods = [
      {
        start: 15.0,
        end: 800.0,
      },
    ];
    if (appliance) {
      let applianceDetails = {
        appliance_type: appliance.replace(/\s/g, ""),
        on_periods: onPeriods,
      };
      details.appliances.push(applianceDetails);
      //console.log(details);
      try {
        const response = await fetch(`${API_URL_AGENT}/add_appliances`, {
          method: "POST",
          body: JSON.stringify(details),
          headers: {
            "Content-Type": "application/json",
            Accept: "application/json",
            Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
          },
          credentials: "include",
        });
        const fdata = await response.json();
        data = fdata;
        //console.log(fdata);
        if (fdata.message == "Succesfully added appliances") {
          document.getElementById("addappliancemodal").showModal();
        }
        // console.log("Data received from user details is: ", data);
      } catch (error) {
        console.log(
          "There was an error with the add appliance endpoint: ",
          error
        );
      }
    } else {
      console.log("Appliance was not selected.");
    }
  }

  async function addGenerator() {
    let details2 = {
      email: email,
      node_id: selectedNodeID,
      generators: [],
    };

    //default is 08:00 - 18:00 (10 hour lapse)
    let onPeriods = {
      start: 28800.0,
      end: 64800.0,
    };

    if (generator && category) {
      //console.log(generator + " " + category);
      let generatorDetails = {
        generator_type: { [generator]: category },
        on_periods: [onPeriods],
      };
      details2.generators.push(generatorDetails);
      //details2.generators.generator_type.push(onPeriods);
      //console.log(details2);
      try {
        const response = await fetch(`${API_URL_AGENT}/add_generators`, {
          method: "POST",
          body: JSON.stringify(details2),
          headers: {
            "Content-Type": "application/json",
            Accept: "application/json",
            Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
          },
          credentials: "include",
        });
        const fdata = await response.json();
        data = fdata;
        if (fdata.message == "Succesfully added generators") {
          document.getElementById("addgeneratormodal").showModal();
        }
        // console.log("Data received from add gen endpoint: ", data);
      } catch (error) {
        console.log(
          "There was an error with the add generator endpoint: ",
          error
        );
      }
    }
  }

  async function addGeneratorWithTime() {
    //conversion of time input
    let startHoursMinutes = intervalStart.split(":");
    let endHoursMinutes = intervalEnd.split(":");

    if (startHoursMinutes[0] > endHoursMinutes[0]) {
      //start is after end (invalid input)
      //show error message
      let errortime = document.getElementById("errorTime");
      errortime.style.display = "block";
      setTimeout(() => {
        errortime.style.display = "none";
      }, 3000);
      return;
    } else if (startHoursMinutes[0] == endHoursMinutes[0]) {
      if (startHoursMinutes[1] >= endHoursMinutes[1]) {
        //start time is either greater than or equal to endTime (invalid input)
        let errortime = document.getElementById("errorTime");
        errortime.style.display = "block";
        setTimeout(() => {
          errortime.style.display = "none";
        }, 3000);
        return;
      }
    }

    intervalStartSeconds =
      startHoursMinutes[0] * 3600 + startHoursMinutes[1] * 60;
    intervalEndSeconds = endHoursMinutes[0] * 3600 + endHoursMinutes[1] * 60;

    let details2 = {
      email: email,
      node_id: selectedNodeID,
      generators: [],
    };

    let onPeriods = {
      start: intervalStartSeconds,
      end: intervalEndSeconds,
    };

    if (generator && category) {
      //console.log(generator + " " + category);
      let generatorDetails = {
        generator_type: { [generator]: category },
        on_periods: [onPeriods],
      };
      details2.generators.push(generatorDetails);
      //details2.generators.generator_type.push(onPeriods);
      //console.log(details2);
      try {
        const response = await fetch(`${API_URL_AGENT}/add_generators`, {
          method: "POST",
          body: JSON.stringify(details2),
          headers: {
            "Content-Type": "application/json",
            Accept: "application/json",
            Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
          },
          credentials: "include",
        });
        const fdata = await response.json();
        data = fdata;
        if (fdata.message == "Succesfully added generators") {
          document.getElementById("addgeneratormodal").showModal();
        }
        // console.log("Data received from add gen endpoint: ", data);
      } catch (error) {
        console.log(
          "There was an error with the add generator endpoint: ",
          error
        );
      }
    }
  }

  $: intervalStart = "";
  $: intervalEnd = "";

  $: categoryChosen = false;
  const onChangeGenerator = () => {
    categoryChosen = false;
  };

  const onChangeCategory = () => {
    categoryChosen = true;
  };

  function showTimeInput() {
    document.getElementById("generatortimes").showModal();
  }

  function showAppliances() {
    getCurve();
    document.getElementById("viewappliancemodal").showModal();
  }

  function showGenerators() {
    getCurve();
    document.getElementById("viewgeneratormodal").showModal();
  }

  async function getCurve() {
    const tempEmail = email;
    //console.log(tempEmail);
    //console.log(selectedNodeID);
    try {
      const response = await fetch(`${API_URL_AGENT}/get_curve`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify({
          email: tempEmail,
          node_id: selectedNodeID,
        }),
        credentials: "include",
      });

      const fdata = await response.json();
      //console.log("data received ",fdata)
      if (fdata.message == "Invalid Email or node_id") {
        applianceNames = "There was an issue retrieving your appliances.";
        generatorNames = "There was an issue retrieving your generators.";
        return;
      }
      console.log(fdata);
      let temp = fdata.data.consumption;
      let size = Object.keys(temp).length;
      if(size != 0)
      {
        //applianceNames.clear();
        applianceNames = new Set();
        temp.forEach((item) => {
          applianceNames.add(item.appliance);
        });
        applianceNames = Array.from(applianceNames).join("\n");
      }
      else{
        applianceNames = "You currently do not have any appliances linked to this node.";
      }
      

      
      /*if (applianceNames === "") {
        applianceNames =
          "You currently do not have any appliances linked to this node.";
      }*/

      let temp2 = fdata.data.production;

      generatorNames = temp2.flatMap((item) => {
        return Object.keys(item[0]);
      });

      generatorNames = generatorNames.join("\n");
      if (generatorNames === "") {
        generatorNames =
          "You currently do not have any generators linked to this node.";
      }
    } catch (error) {
      console.log("An error occurred while fetching getCurve data..\n", error);
    }
  }

  function switchAddWot() {
    if (addWot == "a") {
      addWot = "g";
    } else {
      addWot = "a";
    }
  }
</script>

<main class="container mx-auto p-4">
  <div class="md:flex md:flex-row">
    <!--first-->
    <div class="sm:w-1/3 h-[calc(100vh-70px)] flex flex-col">
      <!--Personal Info-->
      <span class="text-3xl font-thin justify-start pl-2">
        Personal Information
      </span>

      <div class="h-1/2 stats stats-vertical w-full">
        <div class="stat">
          <div class="stat-title">Available Credit</div>
          {#if totalamount == null}
            <span class="loading loading-spinner loading-lg"></span>
          {:else}
            <div class="stat-value font-normal">
              {formatCurrency(totalamount)}
            </div>
          {/if}
        </div>

        <div class="stat flex min-w-max py-0 justify-center">
          <button class="btn btn-primary w-6/12" onclick="add_modal.showModal()"
            >Add funds</button
          >
          <button
            class="btn btn-accent w-6/12"
            onclick="remove_modal.showModal()">Withdraw funds</button
          >
        </div>

        <div class="stat">
          <div class="stat-title">Firstname</div>
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
      <span class="text-3xl font-light justify-start pl-2 mt-2">
        Buy Orders
      </span>
      <div
        class="rounded-2xl h-1/3 backdrop-blur-sm bg-white/30 p-2 overflow-y-auto"
      >
        {#if buyorders.length == 0}
          <div class="rounded-xl h-full bg-base-100 flex justify-center">
            <p class="self-center text-2xl font-light">--No Buy Orders--</p>
          </div>
        {:else}
          {#each buyorders as buyorder}
            <div class="rounded-2xl min-w-1/3 bg-base-100 mb-2">
              <div class="p-5">
                <h2 class="card-title">Buy order</h2>
                <p>
                  Filled units: {buyorder.filled_units.toFixed(1) + "Wh"}<br />
                  Price: {formatCurrency(buyorder.max_price)}<br />
                  Units bought: {Intl.NumberFormat().format(
                    buyorder.sought_units
                  ) + "Wh"}<br />
                </p>
                <div class="card-actions">
                  <progress
                    class="progress progress-primary"
                    value={buyorder.filled_units}
                    max={buyorder.sought_units}
                  ></progress>
                </div>
              </div>
            </div>
          {/each}
        {/if}
      </div>
      <!-- change funds modals -->
      <dialog id="add_modal" class="modal">
        <div class="modal-box">
          <h3 class="text-lg font-bold">Add funds</h3>
          <p class="py-4">Please enter an amount you would like to add.</p>
          <div class="form-control mt-4">
            <input
              class="input input-bordered"
              type="number"
              placeholder="Amount"
              required
              bind:value={amount}
            />
          </div>

          <div class="modal-action">
            <form method="dialog">
              <button class="btn bg-green-600" on:click={addFunds}
                >Continue</button
              >
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
            <input
              class="input input-bordered"
              type="number"
              placeholder="Amount"
              required
              bind:value={withdrawamount}
            />
          </div>
          <div class="modal-action">
            <form method="dialog">
              <button class="btn bg-green-600" on:click={withdrawFunds}
                >Continue</button
              >
              <button class="btn bg-red-500">Cancel</button>
            </form>
          </div>
        </div>
      </dialog>
    </div>

    <!--second-->
    <div class="sm:w-1/3 h-[calc(100vh-70px)] mx-4 flex flex-col">
      <!--Nodes-->
      <span class="basis text-3xl font-thin justify-start pl-2">
        Your Nodes
      </span>
      <div class="h-1/2 flex flex-col">
        <div
          class="rounded-2xl h-full p-2 backdrop-blur-sm bg-white/30 overflow-y-auto"
        >
          {#if nodes.length == 0}
            <div class="rounded-xl h-full bg-base-100 flex justify-center">
              <p class="self-center text-2xl font-light">--No Nodes--</p>
            </div>
          {:else}
            {#each nodes as node}
              {#if node.name == nodeNameDetail}
                <div
                  class="card card-side border-4 border-primary min-w-1/3 bg-base-100 mb-2"
                >
                  <figure class="w-1/4 p-3 pr-0">
                    <img src="../src/images/house.png" alt="House node" />
                  </figure>
                  <div class="card-body pb-4 px-4">
                    <h2 class="card-title font-light text-2xl">{node.name}</h2>
                    <div class="card-actions justify-end">
                      <button
                        class="btn btn-primary"
                        on:click={() => {
                          fetchNodeDetails(node.node_id);
                        }}>Details</button
                      >
                    </div>
                  </div>
                </div>
              {:else}
                <div
                  class="card card-side border-4 border-base-100 min-w-1/3 bg-base-100 mb-2"
                >
                  <figure class="w-1/4 p-3 pr-0">
                    <img src="../src/images/house.png" alt="House node" />
                  </figure>
                  <div class="card-body pb-4 px-4">
                    <h2 class="card-title font-light text-2xl">{node.name}</h2>
                    <div class="card-actions justify-end">
                      <button
                        class="btn btn-primary"
                        on:click={() => {
                          fetchNodeDetails(node.node_id);
                        }}>Details</button
                      >
                    </div>
                  </div>
                </div>
              {/if}
            {/each}
          {/if}
        </div>

        <!--Add New node-->
        <div class="rounded-2xl min-w-1/3 bg-base-100 mt-2 p-3">
          <div class="w-full">
            <button class="btn btn-outline w-full" on:click={createModal}
              >Add a New Node</button
            >
          </div>
        </div>
      </div>

      <!--Sell orders-->
      <span class="text-3xl font-light justify-start pl-2 mt-2">
        Sell Orders
      </span>
      <div
        class="rounded-2xl h-1/3 backdrop-blur-sm bg-white/30 p-2 overflow-y-auto"
      >
        {#if sellorders.length == 0}
          <div class="rounded-xl h-full bg-base-100 flex justify-center">
            <p class="self-center text-2xl font-light">--No Sell Orders--</p>
          </div>
        {:else}
          {#each sellorders as sellorder}
            <div class="rounded-2xl min-w-1/3 bg-base-100 mb-2">
              <div class="p-5">
                <h2 class="card-title">Sell order</h2>
                <p>
                  Claimed Units: {sellorder.claimed_units.toFixed(1) + "Wh"}<br
                  />
                  Offered Units: {sellorder.offered_units.toFixed(1) + "Wh"}<br
                  />
                  Price: {formatCurrency(sellorder.min_price)}<br />
                </p>
                <div class="card-actions">
                  <progress
                    class="progress progress-accent"
                    value={sellorder.claimed_units}
                    max={sellorder.offered_units}
                  ></progress>
                </div>
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <!-- new node modals -->
      <dialog id="mapModal" class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg">Add a Node</h3>
          <form class="">
            <div class="form-control mt-4">
              <input
                class="input input-bordered"
                type="text"
                placeholder="Name"
                bind:value={nodeName}
              />
            </div>
            <!-- <div class="form-control mt-4">
              <input class="input input-bordered" type="text" placeholder="Latitude" bind:value={nodeLatitude}>
            </div>
            <div class="form-control mt-4">
              <input class="input input-bordered" type="text" placeholder="Longtitude" bind:value={nodeLongitude}>
            </div> -->
            <div class="form-control mt-4">
              <Map onMapClick={handleMapClick} />
            </div>

            <div class="form-control mt-4">
              <button class="btn btn-primary" on:click={createNode}
                >Confirm</button
              >
            </div>
          </form>
        </div>

        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>
    </div>

    <!--third-->
    <div class="sm:w-1/3 sm:h-full">
      {#if nodeNameDetail != ""}
        <span class="text-3xl font-thin justify-start pl-2">
          Node Details
        </span>
        <div class="h-5/6">
          <div class="stats stats-vertical w-full">
            <div class="stat">
              <div class="stat-title">Node</div>
              <div class="stat-value font-light">{nodeNameDetail}</div>
            </div>
            <!-- flex min-w-max py-0 justify-center -->
            <div class="stat flex w-full py-0 justify-center mb-2 mt-2">
              <button
                class="btn btn-primary w-6/12"
                on:click={() => {
                  sessionStorage.setItem("node_id", selectedNodeID);
                  sessionStorage.setItem("node_name", nodeNameDetail);
                  //reroute to market
                  goto("../Main/BiddingMarket");
                }}>Transact with this node</button
              >
              <button
                class="btn btn-error w-6/12"
                on:click={() => {
                  document.getElementById("removeNodeConfirmation").showModal();
                }}>Remove node</button
              >
            </div>

            <div class="stat">
              <div class="stat-title">Node Location</div>
              <div class="stat-value font-light">
                {nodeLatitudeDetail < 0
                  ? nodeLatitudeDetail.toFixed(3) * -1 + "S "
                  : nodeLatitudeDetail.toFixed(3) + "N "}
                {nodeLongitudeDetail < 0
                  ? nodeLongitudeDetail.toFixed(3) * -1 + "W "
                  : nodeLongitudeDetail.toFixed(3) + "E "}
              </div>
            </div>

            <div class="stat">
              <div class="stat-title">Available Consumption</div>
              <div class="stat-value font-light">
                {Intl.NumberFormat().format(nodeToConsume)} Wh
              </div>
            </div>

            <div class="stat">
              <div class="stat-title">Pending Generation</div>
              <div class="stat-value font-light">
                {Intl.NumberFormat().format(nodeToProduce)} Wh
              </div>
            </div>
            <div
              id="viewbuttons"
              class="stat flex w-full justify-center mt-2 mb-2"
            >
              <button class="btn btn-primary w-6/12" on:click={showAppliances}
                >View Appliances</button
              >
              <button class="btn btn-primary w-6/12" on:click={showGenerators}
                >View Generators</button
              >
            </div>
          </div>

          <div class="flex-col min-w-3/4 bg-base-100 rounded-2xl p-5 my-2">
            {#if addWot == "a"}
              <div class="form-control mb-1">
                <span class="label text-3xl font-thin overflow-y-auto">
                  Add an Appliance
                  <button on:click={switchAddWot} class="btn btn-primary"
                    >To Generator</button
                  >
                </span>
                <select
                  bind:value={appliance}
                  class="select select-bordered max-h-40 overflow-y-auto my-2"
                >
                  <option value="" disabled selected>Select an appliance</option
                  >
                  {#each appliances as appliance}
                    <option value={appliance}>{appliance}</option>
                  {/each}
                </select>
                <button
                  on:click={addAppliance}
                  class="btn btn-primary my-2"
                  disabled={!appliance}>Add Appliance</button
                >
              </div>
            {:else}
              <!-- selecting category  -->
              <div class="form-control mb-1">
                <span class="label text-3xl font-thin overflow-y-auto">
                  Add a Generator
                  <button on:click={switchAddWot} class="btn btn-primary"
                    >To Application</button
                  >
                </span>
                <select
                  bind:value={generator}
                  class="select select-bordered max-h-40 overflow-y-auto my-2"
                  on:change={onChangeGenerator}
                >
                  <option value="" disabled selected>Select a generator</option>
                  {#each uniqueGens as type}
                    <option value={type}>{type}</option>
                  {/each}
                </select>

                <select
                  bind:value={category}
                  class="select select-bordered max-h-40 overflow-y-auto mt-4"
                  disabled={!generator}
                  on:change={onChangeCategory}
                >
                  <option value="" disabled selected>Select a category</option>
                  {#each generators.filter((g) => g.type === generator) as { category }}
                    <option value={category}>{category}</option>
                  {/each}
                </select>
                <button
                  on:click={showTimeInput}
                  class="btn btn-primary mt-4"
                  disabled={!categoryChosen}>Add Generator</button
                >
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
  <!-- confirm remove node modal -->

  <dialog id="removeNodeConfirmation" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Confirmation</h3>
      <p>
        Are you sure you want to permanently delete this node from your account?
      </p>
      <div class="modal-action">
        <button class="btn btn-error" on:click={removeNode(selectedNodeID)}
          >Yes</button
        >
        <button
          class="btn btn-outline"
          on:click={() => {
            document.getElementById("removeNodeConfirmation").close();
          }}>No</button
        >
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <!-- confirm change funds modals -->

  <dialog id="addfundsconfirmation" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">You have successfully added funds!</h3>
      <p>
        You have successfully added {formatCurrency(amount)} to your account.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

  <dialog id="removefundsconfirmation" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Withdrawal of funds successful!</h3>
      <p>
        You have successfully withdrew {formatCurrency(withdrawamount)} from your
        account.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

  <dialog id="removefundsrejection" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Withdrawal of funds was unsuccessful.</h3>
      <p>
        Withdrawal of {formatCurrency(withdrawamount)} was unsuccessful. Please check
        your balance.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

  <dialog id="addfundsrejection" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Addition of funds unsuccessful.</h3>
      <p>
        Addition of {formatCurrency(amount)} was unsuccessful. Please enter a valid
        value.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button on:click={nullifyValues}>close</button>
    </form>
  </dialog>

  <!-- add appliance modal  -->
  <dialog id="addappliancemodal" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Addition successful.</h3>
      <p>
        Addition of {appliance} was successful.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <!-- add generator modal  -->
  <dialog id="addgeneratormodal" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">Addition successful.</h3>
      <p>
        Addition of {generator} was successful.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <!-- view generator modal  -->
  <dialog id="viewgeneratormodal" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">List of generators ({nodeNameDetail})</h3>
      <p>
        {generatorNames}
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <!-- view appliance modal  -->
  <dialog id="viewappliancemodal" class="modal">
    <div class="modal-box">
      <h3 class="font-bold text-lg">List of appliances ({nodeNameDetail})</h3>
      <p>
        {applianceNames}
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <!-- time input modal -->
  <dialog id="generatortimes" class="modal">
    <div class="modal-box">
      <span class="text-lg font-bold">Operating Time Interval</span>
      <p class="py-4">
        Please enter the typical time periods your generator is running.
      </p>
      <div class="form-control mt-4 grid grid-cols-2 gap-4">
        <div class="grid grid-rows-2">
          <label for="start">Start-time:</label>
          <input
            id="start"
            class="input input-bordered"
            type="time"
            bind:value={intervalStart}
            on:change={console.log(intervalStart)}
          />
        </div>
        <div class="grid grid-rows-2">
          <label for="end">End-time:</label>
          <input
            id="end"
            class="input input-bordered"
            type="time"
            bind:value={intervalEnd}
          />
        </div>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-success" on:click={addGeneratorWithTime}
            >Continue</button
          >
          <button class="btn btn-primary" on:click={addGenerator}>Skip</button>
        </form>
      </div>
    </div>
  </dialog>

  <div class="toast toast-bottom toast-center hidden" id="errorToast">
    <div class="alert alert-error">
      <div>
        <span>Error: Please make sure to select a location on the map.</span>
      </div>
    </div>
  </div>

  <div class="toast toast-bottom toast-center hidden" id="errorNodeName">
    <div class="alert alert-error">
      <div>
        <span
          >Error: Please make sure to enter a name when creating a new node.</span
        >
      </div>
    </div>
  </div>

  <div class="toast toast-bottom toast-center hidden" id="errorTime">
    <div class="alert alert-error">
      <div>
        <span>Error: Please make sure to select valid time periods. </span>
      </div>
    </div>
  </div>
</main>

<style>
  input[type="number"] {
    -moz-appearance: textfield;
  }
  input[type="number"]::-webkit-inner-spin-button,
  input[type="number"]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  /* option to take out the lines from stat class */

  /* .stat {
    border: none; 
    
  } */
</style>
