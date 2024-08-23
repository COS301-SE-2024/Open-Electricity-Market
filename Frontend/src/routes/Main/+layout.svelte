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
    const session = sessionStorage.getItem('Token');
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

  function logout() {
    Cookies.remove("session_id");
    window.location.href = '/login';
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
        <li class="px-2"><a class=" btn-ghost w-22" href="/Main/Dashboard">Dashboard</a></li>
      </ul>
    </div>
      
    <div class="navbar-end">


  <ul class="menu menu-horizontal px-3 ">
    <!-- <button class="bg-slate-800 " on:click={showModal}>Help</button> -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-missing-attribute -->
    <li class="px-2"><a class="w-22 btn-ghost" on:click={showModal}>Help</a></li>
  </ul>
      <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost rounded-btn">Account</div>
        <ul class="menu dropdown-content bg-base-100 rounded-box z-[1] mt-4 w-52 p-2 shadow">
          <button class="btn" onclick="removeaccount_modal.showModal()">Remove Account</button>
          <button class="btn mt-2" on:click={logout}>Log out</button>
        </ul>
      </div>

    </div>
  </div>


  <dialog id="my_modal_dash" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Dashboard</h3>
      <p class="py-4">This is the central hub for controlling your nodes on the grid. <br>
        You can see your details, such as credit, on the left, and a list of your nodes and buy orders on the right. <br>
        If you plan on buying electricity, be sure to start by adding some funds to your account first. <br>
        Your credit is also where you will receive money for any electricity you sell, and you can withdraw from this at any time. <br>
        <br>
        Clicking on the "details" button on any of your nodes will open up more information about them, such as the amount of electricity it is allowed to consume/needs to produce. <br>
        Click on the "Transact with this node" button to go to the market page, where you can be part of our open market.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>

  <dialog id="help_modal" class="modal">  
    <div class="modal-box">
      <h3 class="font-bold text-lg ">Marketplace</h3>
      <p class="py-4">
        The marketplace is designed to be much like any other trading website, where you can observe recent activity in the form of a price graph. <br>
        Here you can place buy orders, or sell your excess power to someone else connected to the grid.
      </p>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>
</header>

<main id="main" class="container mx-auto mt-8">
  {#if loggedIn}
    <slot />
  {/if}


  <dialog id="removeaccount_modal" class="modal">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Delete Account</h3>
      <p class="py-4">Are you sure you want to delete your account?</p>
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