<script>
  import { onMount } from "svelte";
  import Chart from "$lib/Components/Chart.svelte";
  

  let data = {};
  let interval; 
  let advancedView = false; 
  let numDecimals = 2; 

  onMount(async () => {
    await fetchStart();
    await fetchData();
    interval = setInterval(fetchData, 10000);

    //return function runs when the component is unmounted 
    return() => {
      clearInterval(interval);
     
    };
  });

  async function fetchStart() {

      try {
        const response = await fetch("http://localhost:8000/start", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
  }
    catch(error){
      console.log("There was an error sending a post to /start endpoint.");
    }
  };
 async function fetchData() {

      try {
        const response = await fetch("http://localhost:8000/overview", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      }

    });
        console.log("request being sent...");
        console.log(response);
        // const response = fetch("http://localhost:8000");
        const fdata = await response.json();
        console.log(data);
        //Voltage 1,2,3 as well as price
        data = fdata; 
      } catch (error) {
        console.log("There was an error fetching the JSON for the overview..", error);
      }
  };


  function setAdvancedView(){
    advancedView = !advancedView;
    if(advancedView){
      numDecimals = 7; 
    }
    else{
      numDecimals = 2;  
    }
  }

  function createModal(){
    
  }


</script>

<main class="container mx-auto">

  <div class="max-w-min">
    <div class="stats stats-vertical shadow"> 
      <div class="stat">
        <div class="stat-title">Available Credit</div>
        <div class="stat-value">31K</div>
        <div class="stat-desc">Jul 1st - Aug 1st</div>
      </div>
    
      <div class="stat">
        <div class="stat-title">My Nodes</div>
        <div class="stat-value">4,200</div>
        
      </div>
    
      <div class="stat">
        <div class="stat-title">Pending Buy Orders</div>
        <div class="stat-value">1,200</div>
        
      </div>
    </div>
  </div>

  <div class="max-w-min min-h-fit">
    <div class="card bg-base-100 w-60  shadow-xl">
    <figure class="px-10 pt-10">
      <img
        src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
        alt="Shoes"
        class="rounded-xl" />
    </figure>
    <div class="card-body items-center text-center">
      <h2 class="card-title">New Node</h2>
        <div class="card-actions items-center text-center">
          <button class="btn btn-primary" on:click={createModal}>Add a Node</button>
        </div>
      </div>
    </div>


    
  </div>

</main>

