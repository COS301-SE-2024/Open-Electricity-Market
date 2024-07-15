
<script>
     
    import { onMount, onDestroy } from 'svelte';
    import { browser } from '$app/environment';
    import Chart from './Chart2.svelte';
    import {tick} from 'svelte';
    
    let mapContainer;
    let map;
    let lm; 
    let chartdata = [];
    let interval; 
    let data = {};


    
    onMount(async () => {
       if(browser) {
          const leaflet = await import('leaflet');
          map = leaflet.map(mapContainer).setView([-26.1925013,28.0100383], 13);
    
    // Add the tile layer
    leaflet.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', { attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'}).addTo(map);
    
    // Add a marker to the map
    lm = leaflet.marker([-26.1925013,28.0100383]).addTo(map);
    
    lm.on('click', () => showModal());
    



    
       }
       await fetchData();
       interval = setInterval(fetchData, 2000);
    });

    function onClick(e){
      e.bindPopup("this is some text");
    }
    
    onDestroy(async () => {
       if(map) {
          map.remove();
       }
    });

     async function fetchData() {
    try {
      const response = await fetch("http://localhost:8000/info", {
        method: "POST", 
        headers: {
          'Content-Type': 'application/json' 
        }
      });
      console.log("Request being sent...");
      const fdata = await response.json();
      console.log("Fetched data:", fdata);
      data = {
        ...fdata,
        Consumers: fdata.Consumers.map(item => JSON.parse(item)),
        Generators: fdata.Generators.map(item => JSON.parse(item)),
        Transformers: fdata.Transformers.map(item => JSON.parse(item)),
        "Transmission Lines": fdata["Transmission Lines"].map(item => JSON.parse(item))
      };
      // chartdata = data[Consumers.Voltage.Phase1];
      
    } catch (error) {
      console.log("There was an error fetching the JSON for the overview:", error);
    }
  }

    async function showMarkerPopup(marker){
      await tick(); 
      const popupContent = document.createElement('div');
      new Chart({
         target: popupContent,
         props: {
            data: [5,5,3],
            labels: [1,2,3]
         }
      });
      marker.bindPopup(popupContent).openPopup();
    }



   async function showModal(){
      
        document.getElementById("test_modal").showModal();
      
    }

    function extractChartData(data){
        let chartData = [];
        if(data.Consumers){
            chartData = data.Consumers[0].Voltage["Phase 1"];
        }
        return chartData; 
    }

   


    </script>
    
    
    <main>
       <div bind:this={mapContainer}></div>
    </main>
    
    <style>
       @import 'leaflet/dist/leaflet.css';
       div {
       height: 500px;
       }
    </style>



    <dialog id="test_modal" class="modal">  
        <div class="modal-box">
          <h3 class="font-bold text-lg ">Voltage</h3>
          <Chart {data}/>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

