<script>
    import "../../app.css";
    import { page } from '$app/stores';
    import { derived } from "svelte/store";

    let activebutton = '';

    const currentpath = derived(page, $page => $page.url.pathname);

    $: activebutton = $currentpath;

    function showModal(){
        if(activebutton=="/Main/GridSimulation"){
            document.getElementById("my_modal_grid").showModal(); 
        }
        else if (activebutton=="/Main/Dashboard"){
            document.getElementById("my_modal_dash").showModal();
        }
        else if(activebutton=="/Main/BiddingMarket"){
            document.getElementById("help_modal").showModal();
        }

    }

  </script>
  
<style>
  nav a {
    margin-right: 1rem;
  }
  nav a:focus{
    font-weight: bold; 
  }
  .active{
    font-weight: bold; 
  }

  </style>
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
<html lang="en" >
  <header>
    <div class="navbar bg-black rounded-2xl">
      <div class="navbar-start">  
        <span class="card text-xl px-2">Open Electricity Market</span>
      </div>
        
      <div class="navbar-center hidden lg:flex">
        <ul class="menu menu-horizontal px-1">
          <li class="px-2"><a class="w-28 justify-center {activebutton == '/Main/GridSimulation' ? 'bg-zinc-900' : 'bg-slate-800'}" href="/Main/GridSimulation">Grid</a></li>
          <li class="px-2"><a class="w-28 justify-center {activebutton == '/Main/BiddingMarket' ? 'bg-zinc-900' : 'bg-slate-800'}" href="/Main/BiddingMarket">Market</a></li>
          <li class="px-2"><a class="w-28 justify-center {activebutton == '/Main/Dashboard' ? 'bg-zinc-900' : 'bg-slate-800'}" href="/Main/Dashboard">Dashboard</a></li>
        </ul>
      </div>
        
      <div class="navbar-end">
        <ul class="menu menu-horizontal px-3 ">
          <!-- <button class="bg-slate-800 " on:click={showModal}>Help</button> -->
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <!-- svelte-ignore a11y-missing-attribute -->
          <li class="px-2"><a class="w-22 bg-slate-800" on:click={showModal}>Help</a></li>

          <dialog id="my_modal_dash" class="modal">  
            <div class="modal-box">
              <h3 class="font-bold text-lg ">Dashboard Page</h3>
              <p class="py-4">The dashboard page contains metrics that you may use in order to help you decide 
                on when the best time to buy electricity is. 
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

          <li class = "px-2"><a class=" bg-slate-800 w-22" href="/Main/Dashboard">Profile</a></li>

          <!-- theme controller: -->
          <li class = "px-2">
            <label class="flex cursor-pointer gap-2 h-9">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round">
                <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
              </svg>
              <input type="checkbox" value="customLightTheme" class="toggle theme-controller" />
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round">
                <circle cx="12" cy="12" r="5" />
                <path
                  d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4" />
              </svg>
            </label>
          </li>
        </ul>
      </div>
    </div>

  </header>

  <main class="container mx-auto mt-8">
      <slot />
  </main>
</html>
