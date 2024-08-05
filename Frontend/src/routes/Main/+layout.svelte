<script>
  import "../../app.css";
  import {page} from '$app/stores';
  import {derived} from "svelte/store";
  import Cookies from 'js-cookie';
  import {onMount} from "svelte";

  let loggedIn = false; 

  let activebutton = '';

  const currentpath = derived(page, $page => $page.url.pathname);

  $: activebutton = $currentpath;

  function showModal(){
    if(activebutton=="/public/GridSimulation"){
        document.getElementById("my_modal_grid").showModal(); 
    }
    else if (activebutton=="/Main/Dashboard"){
        document.getElementById("my_modal_dash").showModal();
    }
    else if(activebutton=="/Main/BiddingMarket"){
        document.getElementById("help_modal").showModal();
    }

  }

  onMount(() => {
    const session = Cookies.get('session_id');
    // console.log("Session id is: ", session);
    if(session){
      loggedIn = true; 
    }
    else{
      loggedIn = false; 
      window.location.href = '/login';
    }
    // loggedIn = session === 'loggedIn';
  });

  async function removeAccount(){
    let data;
    try {
      const response = await fetch("http://localhost:8001/remove_account", {
        method: "POST",
        headers: {
          'Content-Type': 'application/json'
        },
        credentials: "include",
      });
      data = await response.json();
      // console.log("Data received from remove account endpoint: ", data);

    } catch (error) {
      console.log("There was an error calling remove account:", error);
    }

    if(data.message == "Account successfully deleted"){
      Cookies.remove("session_id");
      window.location.href = '/login';
    }


  }
  </script>
  
<!--   
  <header class="bg-gray-800 text-white p-4 ">
    <nav class="container mx-auto">
      <a href="/Main/Dashboard" class = {activebutton == '/Main/Dashboard' ? 'active' : ''}>Dashboard</a>
      <a href="/Main/GridSimulation" class = {activebutton == '/Main/GridSimulation' ? 'active' : ''}>Grid Simulation</a>
      <a href="/Main/BiddingMarket" class = {activebutton == '/Main/BiddingMarket' ? 'active' : ''}>Bidding Market</a>
    </nav>
  </header>
  
  <main class="container mx-auto mt-8">
    <slot />
  </main> -->
<header>
  <div class="navbar bg-base-100 border-b border-accent">
    <div class="navbar-start">  
      <a class="btn btn-ghost text-xl" href="/">Amplify</a>
      <span class="text-xl pl-4"> {activebutton == '/public/GridSimulation' ? "Simulation" : 
      activebutton == '/Main/BiddingMarket' ? "Marketplace" : 
      activebutton == '/Main/Dashboard' ? "Dashboard" : ""} </span>
    </div>
      
    <div class="navbar-center hidden lg:flex">
      <ul class="menu menu-horizontal px-1">
        <li class="px-2"><a class="w-28 justify-center btn-ghost" href="/public/GridSimulation">Grid</a></li>
        <li class="px-2"><a class="w-28 justify-center btn-ghost" href="/Main/BiddingMarket">Market</a></li>
      </ul>
    </div>
      
    <div class="navbar-end">


  <ul class="menu menu-horizontal px-3 ">
    <!-- <button class="bg-slate-800 " on:click={showModal}>Help</button> -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-missing-attribute -->
    <li class="px-2"><a class="w-22 btn-ghost" on:click={showModal}>Help</a></li>

    <li class = "px-2"><a class=" btn-ghost w-22" href="/Main/Dashboard">Dashboard</a></li>
  </ul>
      <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost rounded-btn">Account</div>
        <ul class="menu dropdown-content bg-base-100 rounded-box z-[1] mt-4 w-52 p-2 shadow">
          <button class="btn" onclick="removeaccount_modal.showModal()">Remove Account</button>
        </ul>
      </div>

    </div>
  </div>


  <dialog id="my_modal_dash" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Dashboard Page</h3>
      <p class="py-4">Here you can see details about your account and interactions on the grid. 
        Go to the marketplace to purchase or sell electricity for any of your nodes. 
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <dialog id="my_modal_grid" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Grid Simulation Page</h3>
      <p class="py-4">The grid simulation page contains an overview of the current 
        state of the electrical grid. 
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <dialog id="help_modal" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Bidding Market Page</h3>
      <p class="py-4">Click the button on the 'Advertise Here' card to enter the number of units you want to sell, and the price you wish to sell them for.</p>
      <p class="py-4">You can click the button on any of the advertisements to buy one unit of electricity from them.</p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>
</header>

<main class="container mx-auto mt-8">
  {#if loggedIn}
    <slot />
  {:else}
    <script>
        // 
    </script>
  {/if}


  <dialog id="removeaccount_modal" class="modal">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Delete Account</h3>
      <p class="py-4">Are you sure you would like to delete your account?</p>
      <div class="modal-action">
        <form method="dialog">
          <!-- if there is a button in form, it will close the modal -->
          <button class="btn bg-red-500" on:click={removeAccount}>Delete</button>
          <button class="btn bg-gray-600">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
</main>