<svelte:window bind:scrollY />

<svelte:head>
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</svelte:head>

<script>
  import Map from "$lib/Components/MapLanding.svelte";
  import { fade } from 'svelte/transition'
  import market_image from "$lib/assets/pexels-energepic-com-27411-159888.jpg"
  import strategy_image from "$lib/assets/pexels-pixabay-163064.jpg"
  import breaker_image from "$lib/assets/troy-bridges-maXnRLszYY0-unsplash.jpg"
  import { onMount } from "svelte";

  let scrollY
  let element
  let height
  let loggedIn = false;

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

  onMount(() => {
    const session = sessionStorage.getItem('Token');
    // console.log("Session id is: ", session);
    if(session){
      loggedIn = true; 
    }
  });

  function scrollDown() {
    document.getElementById("Second").scrollIntoView({behavior: "smooth"});
  }

  function scrollUp() {
    document.getElementById("First").scrollIntoView({behavior: "smooth"});
  }

</script>

<main class ="bg-[url('../src/images/jimmy-chang-xnpq29vhHms-unsplash.jpg')] bg-cover bg-fixed !scroll-smooth pb-6">


<section id="First" class="flex justify-center h-screen items-center">
  <div class="sm:card w-full sm:max-w-lg shadow-xl glass ">
    <div class="card-body items-center text-center ">
      <h2 class="text-4xl font-bold  card-title text-white w-128">Transform the way you manage energy.</h2>
      <br>
      <div class="card-actions ">
        {#if loggedIn}
        <a class="btn btn-outline text-xl text-white" href="/Main/Dashboard">Dashboard</a>
        {:else}
        <a class="btn btn-outline text-xl text-white" href="/login">Sign in</a>
        {/if}
        <a class="btn btn-outline text-xl text-white" href="/public/GridSimulation">Simulation</a>
      </div>
      <br>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="flex flex-col w-max justify-center items-center" on:click={scrollDown}>
        <div class = "text-white">scroll for more info</div>
        <svg width="32" height="32" class="pulse-1 -mb-5" fill="#ffffff" viewBox="0 0 256 256">
          <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path>
        </svg>

        <svg width="32" height="32" class="pulse-2 -mb-5" fill="#ffffff" viewBox="0 0 256 256">
          <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path>
        </svg>

        <svg width="32" height="32" class="pulse-3 -mb-5" fill="#ffffff" viewBox="0 0 256 256">
          <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path>
        </svg>
      </div>
    </div>
  </div>
</section>

<section id="Second">
  <div bind:this="{element}" bind:clientHeight="{height}" class="flex flex-row justify-center">
    <div class="sm:basis-5/6 flex-row mx-4" >
      <div class="card sm:card-side glass min-h-72 shadow-xl mt-4" transition:fade>
        <figure class = "max-w-96">
          <img src={market_image} alt="price graph"/>
        </figure>
        <div class="card-body text-white">
          <h2 class="card-title text-4xl">Free market</h2>
          <p>
            Amplify provides users with an open market to buy and sell electricity. Analytic
            tools such as price charts as well as simulations are provided to give users a history of the grid state and help them make
            informed decisions.
          </p>
        </div>
      </div>


      <div class="card sm:card-side glass shadow-xl mt-4 min-h-72">
        <figure class = "max-w-96">
          <img src={strategy_image} alt="complex network" />
        </figure>
        <div class="card-body text-white">
          <h2 class="card-title text-4xl">User asset management</h2>
          <p>Powerful tools are provided for users to manage their own nodes (electrical equipment). This allows
          users to manage there own electricity on a nation wide level.</p>
        </div>
      </div>

      <div class="card sm:card-side glass shadow-xl mt-4 min-h-72">
        <figure class="max-w-96">
          <img src={breaker_image} alt="img" />
        </figure>
        <div class="card-body text-white">
          <h2 class="card-title text-4xl">Controlled Market</h2>
          <p>The market features several control systems to ensure a fair market as well as a stable grid. Price is
          adjusted to ensure the grid load stays<br> within predetermined limits, thus providing a more stable grid. Electricity that is bought has a lifetime in order to avoid hoarding.</p>
        </div>
      </div>

      <div class="card card-side mt-4 -mb-8 sm:hidden">
        <div class="card-body p-0">
          <figure class = "min-w-full rounded-lg">
            <Map/>
          </figure>

        </div>
      </div>

      <div class="card card-side glass shadow-xl mt-4 min-h-72">
        <figure class="sm:w-96">
          <Map/>
        </figure>
        <div class="card-body text-white">
          <h2 class="card-title text-4xl">Real-time grid simulation</h2>
          <p>The market platform is built on top of a accurate electrical grid simulation. This simulation takes
          the following into account:
            <li>Kriscoff's Voltage law</li>
            <li>Kriscoff's Current law</li>
            <li>Inductive impedance</li>
            <li>3 Phase AC (alternating current)</li>
          </p>
        </div>
      </div>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="flex flex-col w-full my-8 pt-4 justify-center items-center" on:click={scrollUp}>
        <div class="rotate-180">
          <svg width="32" height="32" class="pulse-1 -mb-5" fill="#ffffff" viewBox="0 0 256 256">
            <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path>
          </svg>
          
          <svg width="32" height="32" class="pulse-2 -mb-5" fill="#ffffff" viewBox="0 0 256 256">
            <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path>
          </svg>
          
          <svg width="32" height="32" class="pulse-3 -mb-5" fill="#ffffff" viewBox="0 0 256 256">
            <path d="M212.24,100.24l-80,80a6,6,0,0,1-8.48,0l-80-80a6,6,0,0,1,8.48-8.48L128,167.51l75.76-75.75a6,6,0,0,1,8.48,8.48Z"></path>
          </svg>
        </div>
        <div class = "text-white">back to top</div>
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
</style>

