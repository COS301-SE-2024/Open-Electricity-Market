<script>
  import { onMount } from "svelte";
  import { uid } from "uid";
  let interval;

  // console.log(uid());
  let arrayofuids = [];
  var arrayofpricesbought = [];
  // const userid = uid();


  

  onMount(async () => {
    await checkMet();
    interval = setInterval(checkMet, 800);
    
    //return function runs when the component is unmounted
    return () => {
      clearInterval(interval);
    };
  });

  async function removeelement(element){
    for(let i = 0; i<arrayofuids.length; i++){
      let tempy = arrayofuids[i]; 
      if (tempy == element) {
        arrayofuids.splice(i, 1);
      }
    }
  }

  async function checkMet() {
    for (let i = 0; i < arrayofuids.length; i++) {
      const element = arrayofuids[i];
      const response = await fetch(`http://localhost:8001/met/${element}`);
      let text = await response.text();
      // console.log(text);
      if (text == "true") {
        console.log(`Bid met for agent with id of: ${element}`);
        removeelement(element);
      }
    }
    arrayofuids = arrayofuids;
  }

  async function bidFunction() {
    //https://api.coindesk.com/v1/bpi/currentprice.json
    let id = uid();
    arrayofuids.push(id);
    console.log(id);
    console.log(arrayofuids);
    const response = await fetch(`http://localhost:8001/bid/1118/100/${id}`); //have to insert endpoint
    let text = await response.text();
    console.log(text);

    // data = JSON.parse(text)
  }

  async function sellFunction() {
    //https://api.coindesk.com/v1/bpi/currentprice.json
    const response = await fetch("http://localhost:8001/sell/1500"); //have to insert endpoint
    let text = await response.text();
    console.log(text);
    // data = JSON.parse(text)
  }
</script>

<main class="container mx-auto">
  <h1 class="text-2xl font-bold mb-4">Bidding Market</h1>
  <div class="overflow-x-auto">
    <table class="table table-xs">
      <thead>
        <tr>
          <th></th>
          <th>Id</th>
          <th>Price</th>
        </tr>
      </thead>
      
      
      
      {#each arrayofuids as id (id)}
      <tbody>
        <tr>
          <!-- <th>{i+1}</th> -->
          <td>{id}</td>
          <td>R230.45</td>
        </tr>
      </tbody>
      {/each}
    
      
    </table>

    <button on:click={bidFunction} class="btn bg-green-600 mt-20 mr-1 w-1/6"
      >Bid</button
    >
    <button on:click={sellFunction} class="btn bg bg-red-700 w-1/6">Sell</button
    >
  </div>
</main>

<style>
</style>
