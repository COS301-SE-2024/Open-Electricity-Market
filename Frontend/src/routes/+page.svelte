<svelte:window bind:scrollY />

<script>
  import Map from "$lib/Components/MapLanding.svelte";

  let scrollY
  let element
  let height

  $: offset = (element && element.offsetTop)
  $: result = scrollY/(offset+height)*100
</script>

<section>
  <div bind:this="{element}" bind:clientHeight="{height}" class="flex flex-row">
    <div class="basis-1/2">

    </div>
    <div class="basis-1/2">
      <Map class = "w-1/2" />
      <p>Hi</p>
      <p>Hi</p>
      <p>Hi</p>
    </div>

  </div>
</section>

<span class="block fixed text-center left-5 top-1 z-10">
    <h1 class="text-5xl font-bold pt-8">Open Electricity Market.</h1>
    <p class="py-6">Discover a revolutionary platform where you can buy and sell electricity in real-time.</p>
    <a class="btn btn-accent text-xl" href="/login">Sign in</a>
    <a class="btn btn-accent text-xl" href="/Main/GridSimulation">Show me the simulation</a>
</span>



<style>
  :global(*) {
    box-sizing: border-box;
    position: relative;
  }
  :global(body) {
    padding: 0;
  }
  section {
    padding: 100vh 0 200vh;
  }
</style>

<!--<main>-->



<!--    <div slot="background" class="min-h-screen bg-[linear-gradient(to_bottom,rgb(29, 35, 42, 0.8),rgba(29, 35, 42,0)),url('../src/images/background-pylons.jpg')]">-->
<!--      <a class="btn btn-accent text-xl" href="/login">Sign in</a>-->





<!--      &lt;!&ndash;  <Chart />&ndash;&gt;-->
<!--    </div>-->

<!--    <div slot="foreground" class="">-->
<!--    <div class="text-center lg:text-center justify-center pl-20">-->
<!--      &lt;!&ndash; <a class="btn btn-outline text-xl justify-end" href="/login">Sign in</a> &ndash;&gt;-->


<!--      <section class = "h-80"><Map class = "w-1/2" /></section>-->


<!--    </div>-->
<!--    </div>-->
<!--  </Scroller>-->


<!--  <div class="text-center mt-6">-->
<!--    <a class="btn btn-accent text-xl" href="/login">Sign in</a>-->
<!--    <a class="btn btn-accent text-xl" href="/Main/GridSimulation">Show me the simulation</a>-->
<!--  </div>-->



<!--</main>-->

