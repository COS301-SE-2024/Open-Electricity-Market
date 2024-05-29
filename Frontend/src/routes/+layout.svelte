<script>
    import "../app.css";
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { derived } from "svelte/store";

    let activebutton = '';

    const currentpath = derived(page, $page => $page.url.pathname);

    $: activebutton = $currentpath; 

    // $: console.log(activebutton);

    

    


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
  
  <header class="bg-gray-800 text-white p-4">
    <nav class="container mx-auto">
      <a href="/Dashboard" class = {activebutton == '/Dashboard' ? 'active' : ''}>Dashboard</a>
      <a href="/GridSimulation" class = {activebutton == '/GridSimulation' ? 'active' : ''}>Grid Simulation</a>
      <a href="/BiddingMarket" class = {activebutton == '/BiddingMarket' ? 'active' : ''}>Bidding Market</a>
    </nav>
  </header>
  
  <main class="container mx-auto mt-8">
    <slot />
  </main>