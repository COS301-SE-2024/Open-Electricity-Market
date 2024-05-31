<script>
  import { onMount } from "svelte";
  let interval;

  onMount(async () => {
    await checkMet();
    interval = setInterval(checkMet, 800);

    //return function runs when the component is unmounted
    return () => {
      clearInterval(interval);
    };
  });

  async function checkMet(id) {
    const response = await fetch("http://localhost:8001/met/temp");
    let text = await response.text();
    if (text == "true") {
      alert("Bid met");
    }
  }

  async function bidFunction() {
    //https://api.coindesk.com/v1/bpi/currentprice.json
    const response = await fetch("http://localhost:8001/bid/1118/100/temp"); //have to insert endpoint
    let text = await response.text();
    console.log(text);
    // data = JSON.parse(text)
  }

  async function sellFunction() {
    //https://api.coindesk.com/v1/bpi/currentprice.json
    const response = await fetch("http://localhost:8001/sell/2000"); //have to insert endpoint
    let text = await response.text();
    console.log(text);
    // data = JSON.parse(text)
  }
</script>

<main class="container mx-auto">
  <h1 class="text-2xl font-bold mb-4">Bidding Market</h1>

  <button on:click={bidFunction} class="btn bg-green-600">Bid</button>
  <button on:click={sellFunction} class="btn bg bg-red-700">Sell</button>
</main>

<style>
</style>
