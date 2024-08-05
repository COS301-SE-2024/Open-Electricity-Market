<svelte:window bind:scrollY />

<svelte:head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</svelte:head>

<script>
  import Map from "$lib/Components/MapLanding.svelte";
  import ChartLanding from "$lib/Components/ChartLanding.svelte";
  import IntersectionObserver from '$lib/Components/IntersectionObserver.svelte';

  let scrollY
  let element
  let height

  $: offset = (element && element.offsetTop)
//   $: result = scrollY/(offset+height)*100;

    const options = {
      threshold: [0.4, 0.8]
    }

    const callback = (entries) => {
        entries && entries.forEach(entry => {
            if(entry.isIntersecting){
                this.addClassName('visible');
            }
            else{
                this.removeClassName('visible');
            }
        }); 
    }; 

    // const observer = new IntersectionObserver(callback, options);

    // const target = document.querySelector('#Second');
    // observer.observe(target);


</script>

<main class ="bg-[url('../src/images/background-pylons.jpg')] bg-cover bg-fixed !scroll-smooth">


<section id = "First">
    <span class="flex justify-center h-screen items-center ">

    <div class="card max-w-lg shadow-xl glass">
    <div class="card-body items-center text-center">
        <h2 class="text-5xl font-bold  card-title">Transform the way you manage energy.</h2>
            <div class="card-actions">
                 <a class="btn bg-slate-800 text-xl" href="/login">Sign in</a>
                 <a class="btn bg-slate-800 text-xl" href="/public/GridSimulation">Simulation</a>
            </div>
            <div class="belowcard flex flex-col w-max justify-center items-center">

    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" class="pulse-1 -mb-[20px] svelte-1gnar4p" fill="#fff" viewBox="0 0 256 256">
    <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path></svg>

    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" class="pulse-2 -mb-[20px] svelte-1gnar4p" fill="#fff" viewBox="0 0 256 256">
    <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path></svg>

    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" class="pulse-3 -mb-[20px] svelte-1gnar4p" fill="#fff" viewBox="0 0 256 256">
    <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path></svg>

    </div>
    </div>

        
        
    </div>

    
    
    

    </span>
</section>

<section id = "Second">
  <div bind:this="{element}" bind:clientHeight="{height}" class="flex flex-row">
    <!-- <div class="md:basis-1/4 sm:basis-0">

    </div> -->



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


        <div class="card md:card-side glass shadow-xl mt-4 flex justify-center ">
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

  .pulse-1{
     animation: pulse1 2s infinite ease-in-out;
  }

  .pulse-2{
     animation: pulse2 2s infinite ease-in-out;
  }

  .pulse-3{
     animation: pulse 2s infinite ease-in-out;
  }

  @keyframes pulse {
     0% {
        opacity: 1;
    }
    20% {
        opacity:1;
    }
    40% {
        opacity:1;
    }
    60% {
        opacity: 0;
    }
    80% {
        opacity: 0;
    }
    100% {
        opacity: 1;
    }
  }

  @keyframes pulse1 {
    0% {
        opacity: 1;
    }
    20% {
        opacity:0;
    }
    40% {
        opacity:0;
    }
    60% {
        opacity: 1;
    }
    80% {
        opacity: 1;
    }
    100% {
        opacity: 1;
    }
  }

  @keyframes pulse2 {
    0% {
        opacity: 1;
    }
    20% {
        opacity:1;
    }
    40% {
        opacity:0;
    }
    60% {
        opacity: 0;
    }
    80% {
        opacity: 1;
    }
    100% {
        opacity: 1;
    }
  }

    .section {
    opacity: 0;
    transition: opacity 1s ease-out;
    will-change: opacity;
    }
    .section.visible {
    opacity: 1;
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

