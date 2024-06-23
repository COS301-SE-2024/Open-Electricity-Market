<script>
    import "../../app.css";
    import { page } from '$app/stores';
    import { derived } from "svelte/store";

    let activebutton = '';

    const currentpath = derived(page, $page => $page.url.pathname);

    $: activebutton = $currentpath;
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
<main>
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
      <a class="btn bg-slate-800" href="/Main/Dashboard">Profile</a>
    </div>
  </div>
</main>