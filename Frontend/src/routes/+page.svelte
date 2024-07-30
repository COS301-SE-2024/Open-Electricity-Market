<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    import Map from '$lib/Components/MapLanding.svelte';
    import Chart from '$lib/Components/ChartLanding.svelte';
</script>

<main>
  <div class="min-h-min bg-[linear-gradient(to_bottom,rgb(29, 35, 42, 0.8),rgba(29, 35, 42,0)),url('../src/images/background-pylons.jpg')]">
    <div class="text-center lg:text-center justify-center ">
      <!-- <a class="btn btn-outline text-xl justify-end" href="/login">Sign in</a> -->
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