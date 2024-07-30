<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    import Map from '$lib/Components/MapLanding.svelte';
    import Chart from '$lib/Components/ChartLanding.svelte';
</script>

<main>
  <div class="min-h-min bg-[linear-gradient(to_bottom,rgba(0,0,0,0),rgba(255,255,255,1)),url('../src/images/background-pylons.jpg')]">
    <div class="text-center lg:text-center justify-center bg-base-100">
      <h1 class="text-5xl font-bold pt-8">Open Electricity Market.</h1>
      <p class="py-6">Discover a revolutionary platform where you can buy and sell electricity in real-time.</p>
      <Map /> 
      <Chart /> 
      <div class="text-center mt-6">
        <a class="btn btn-accent text-xl" href="/login">Sign in</a>
        <a class="btn btn-accent text-xl" href="/Main/GridSimulation">Show me the simulation</a>
      </div>
    </div>
  </div>
</main>