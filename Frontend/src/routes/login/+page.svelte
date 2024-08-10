<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    import Cookies from 'js-cookie'
    let email = '';
    let password = '';
    let errormessage = "";
    import { API_URL_GRID, API_URL_MARKET, COOKIE_DOMAIN } from '$lib/config.js';

    // RFC 2822 standard email validation pattern
    var emailregex = /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;

    async function login(){

      if (!email.match(emailregex)) {
        errormessage = "Please enter a valid email address";
        return;
      }

      errormessage = "";

      const res = await fetch(`${API_URL_MARKET}/login`, {
        method: "POST", 
        headers: {
          "Content-Type": "application/json",
          "Accept": "application/json",
        },
        body: JSON.stringify({
          "email": email, 
          "password": password
        })
      })
      const json = await res.json()
      //result = JSON.stringify(json)
      console.log(json);
      if(json.message == "User logged in")
      {
        Cookies.set('session_id', json.data.session_id, {path: '/', domain : COOKIE_DOMAIN, sameSite : 'None', secure:true });
        goto("/Main/Dashboard");
      }
      else{
        errormessage = "Invalid Credentials";
      }
      //send to main page
    }

    function showPassword() {
    var input = document.getElementById("pw");
    if (input.type === "password") {
      input.type = "text";
    } else {
      input.type = "password";
    }
  }
</script>

<main>
    <div class="hero min-h-screen" style="background-image: url(https://images.unsplash.com/photo-1510595256055-e44b27efe497?q=80&w=1700&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D);">
        <div class="hero-overlay bg-opacity-60 "></div>
        <div class="hero-content flex-col lg:flex-row-reverse rounded-2xl bg-base-100">
          <div class="text-center lg:text-left">
            <h1 class="text-5xl font-bold">Amplify</h1>
            <p class="py-6">Discover a revolutionary open market platform where you can buy and sell electricity in real-time.</p>
          </div>
          <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
            <form class="card-body">
              <h2 class="text-base font-semibold"> Log in to your account </h2>
              
              <div class="form-control mt-1">
                <input type="email" placeholder="Email" class="input input-bordered" required bind:value={email}/>
              </div>

              <div class="form-control mt-4">
                <input id="pw" type="password" placeholder="Password" class="input input-bordered" required bind:value={password}/>
                <div class="flex ml-2 mt-2">
                  <input on:click={showPassword} id="hs-toggle-password-checkbox" type="checkbox" class="shrink-0 mt-0.5 border-gray-200 rounded text-blue-600 focus:ring-blue-500 dark:bg-neutral-800 dark:border-neutral-700 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800">
                  <label for="hs-toggle-password-checkbox" class="text-sm text-gray-500 ms-3 dark:text-neutral-400">Show password</label>
                </div>
                <!-- <label class="label" for="">
                  <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
                </label> -->
              </div>

              {#if errormessage != ''}
                <h2 class="text-base font-semibold text-black bg-error rounded"> { errormessage } </h2>
              {/if}

              <div class="form-control mt-6">
                <button class="btn btn-primary" on:click={login}>Login</button>
              </div>

              <div class="form-control mt-3"> 
                <!-- <button class="btn btn-primary" on:click={signup}>Create an account</button> -->
                <a class="btn btn-outline btn-primary" href="/signup">Create an account</a>
              </div>
            </form>
          </div>
        </div>
      </div>
</main>