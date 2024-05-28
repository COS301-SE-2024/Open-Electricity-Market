<script>
    import "../app.css";
    import { goto } from '$app/navigation';
  </script>
  
  <style>
    nav a {
      margin-right: 1rem;
    }
  </style>
  
  <header class="bg-gray-800 text-white p-4">
    <nav class="container mx-auto">
      <a href="/" class="text">Home</a>
      <a href="/GridSimulation">Grid Simulation</a>
      <a href="/BiddingMarket">Bidding Market</a>
    </nav>
  </header>
  
  <main class="container mx-auto mt-8">
    <slot />
  </main>