<script>
  async function makebuyrequest(id) {
    try {
      const response = await fetch("http://localhost:8000/consume", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          id: `${id}`,
          load: "2000",
        }),
      });
    } catch (error) {
      console.log(
        "There was an error sending the post request for consume. ",
        error
      );
    }
  }
</script>

<!-- id logic will have to be added later -->
<button on:click={() => makebuyrequest(id)} class="btn bg-green-600 w-28 h-14"
  >Buy</button
>
