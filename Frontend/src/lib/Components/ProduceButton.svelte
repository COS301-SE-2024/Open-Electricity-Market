 
 <script>

    async function makeproducerequest(id) {
         try {
        const response = await fetch("http://localhost:8000/produce", {
      method: "POST", 
      headers: {
        'Content-Type': 'application/json' 
      },
      body:JSON.stringify({
        id:`${id}`,
        supply:'1024'
      })
    });
    }
    catch(error){
        console.log("There was an error sending the post request for produce. ", error);
    }
}

 </script>
 
 <!-- id logic will have to be added later -->
 <button on:click={()=>makeproducerequest(id)} class="btn bg-red-700 w-28 h-14">Produce</button>