<svelte:window bind:scrollY />

<svelte:head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</svelte:head>

<script>
  import Map from "$lib/Components/MapLanding.svelte";
  import ChartLanding from "$lib/Components/ChartLanding.svelte";

  let scrollY
  let element
  let height

  $: offset = (element && element.offsetTop)
  $: result = scrollY/(offset+height)*100
</script>

<main class ="bg-[url('../src/images/background-pylons.jpg')] bg-cover bg-fixed">
<section>
  <div bind:this="{element}" bind:clientHeight="{height}" class="flex flex-row">
    <div class="md:basis-1/4 sm:basis-0">

    </div>



    <div class="md:basis-3/4 sm:basis-1 m-16 md:ml-64 ">
        <div class="card md:card-side glass shadow-xl mt-4">
            <figure class = "max-w-96 ">
                <img src="../src/images/pexels-energepic-com-27411-159888.jpg" alt="price graph"/>
            </figure>
            <div class="card-body text-white">
                <h2 class="card-title">Free market</h2>
                <p>
                    Amplify is built as a free market economy where any user can both buy and sell electricity. Analitic
                    tools such as price charts are provided to give users a history of the economy and help them make
                    informed decisions.
                </p>
            </div>
        </div>


        <div class="card md:card-side glass shadow-xl mt-4">
            <figure class = "max-w-96">
                <img src="../src/images/pexels-pixabay-163064.jpg" alt="complex network" />
            </figure>
            <div class="card-body text-white">
                <h2 class="card-title">User asset managament</h2>
                <p>Powerful tools are provided for users to manage their own nodes (electrical equipment). This empowers
                users to manage there own electricity on a nation wide level.</p>
            </div>
        </div>



        <div class="card md:card-side glass shadow-xl mt-4">
            <figure class = "max-w-96 ">
                <img src="../src/images/troy-bridges-maXnRLszYY0-unsplash.jpg" alt="img" />
            </figure>
            <div class="card-body text-white">
                <h2 class="card-title">Controlled Market</h2>
                <p>The market features several control systems to ensure a fair market and a stable grid. Price is
                adjusted to ensure keep the grid load inside limits, bought electricity expires to avoid hording and so
                forth</p>
            </div>
        </div>

        <div class="card md:card-side glass shadow-xl mt-4">
            <figure class = "min-w-96">
                <Map class = "w-1/2" />
            </figure>
            <div class="card-body text-white">
                <h2 class="card-title">Real time grid simulation</h2>
                <p>The market platform is built on top of a accurate electrical grid simulation. This simulation takes
                the following into account:
                    <li>Kriscoff's Voltage law</li>
                    <li>Kriscoff's Current law</li>
                    <li>Inductive impedance</li>
                    <li>3 Phase AC (alternating current)</li>
                </p>
            </div>
        </div>
    </div>

  </div>
</section>

<span class="block fixed text-center md:left-5 sm:left-1/4 top-16 z-10">

    <div class="card bg-base-100 max-w-lg shadow-xl">
    <div class="card-body items-center text-center">
        <h2 class="text-5xl font-bold  card-title">Amplify</h2>
            <p class="py-6">Discover a revolutionary platform where you can buy and sell electricity in real-time.</p>
            <div class="card-actions">
                 <a class="btn bg-slate-800 text-xl" href="/login">Sign in</a>
                 <a class="btn bg-slate-800 text-xl" href="/public/GridSimulation">Show me the simulation</a>
            </div>
        </div>
    </div>

     <div class="card bg-base-100 shadow-xl mt-4 max-w-lg ">
            <div class="card-body">
                <h2 class="card-title">Who are we?</h2>
                <p>We are a group dedicated to exploring new and better ways of electrical distribution.  </p>
                <h2 class="card-title">What is Amplify?</h2>
                <p>
                    Amplify is a experimental new platform for testing an electrical grid based of a free market
                </p>
            </div>
        </div>
</span>
</main>


<style>
  :global(*) {
    box-sizing: border-box;
    position: relative;
  }
  :global(body) {
    padding: 0;
  }
  section {
    padding: 0vh 0 0vh;
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

