<script>
  import logo from "$lib/assets/Logo.png";
  import { goto } from "$app/navigation";
  import Cookies from "js-cookie";
  let email = "";
  let password = "";
  let errormessage = "";
  import {
    API_URL_GRID,
    API_URL_MARKET,
    COOKIE_DOMAIN,
    API_URL_AGENT,
  } from "$lib/config.js";

  // RFC 2822 standard email validation pattern
  var emailregex =
    /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;

  async function login() {
    if (!email.match(emailregex)) {
      errormessage = "Please enter a valid email address";
      return;
    }

    errormessage = "";

    const res = await fetch(`${API_URL_MARKET}/login`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify({
        email: email,
        password: password,
      }),
    });
    const json = await res.json();
    //result = JSON.stringify(json)
    // console.log(json);
    if (json.message == "User logged in") {
      sessionStorage.setItem("Token", json.data.token);
      await addAgent();
      goto("/Main/Dashboard");
    } else {
      errormessage = "Invalid Username or Password";
      var input = document.getElementById("pw");
      if (password == "") {
        input.classList.add("input-error");
        errormessage = "Please enter your password"
      } else {
        input.classList.remove("input-error");
      }
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

  async function addAgent() {
    let details = {
      email: email,
      password: password,
      token: sessionStorage.getItem("Token"),
    };
    try {
      const res = await fetch(`${API_URL_AGENT}/add_agent`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${sessionStorage.getItem("Token")}`,
        },
        body: JSON.stringify(details),
        credentials: "include",
      });
      const fdata = await res.json();
      console.log("Add agent endpoint: ", fdata);
    } catch (error) {
      console.log(
        "There was an error with the add_agent endpoint during login: ",
        error
      );
    }
  }
</script>

<main>
  <div
    class="hero min-h-screen"
    style="background-image: url(https://images.unsplash.com/photo-1510595256055-e44b27efe497?q=80&w=1700&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D);"
  >
    <div class="hero-overlay bg-opacity-60"></div>
    <div
      class="hero-content flex-col lg:flex-row-reverse rounded-2xl bg-base-100"
    >
      <div class="text-center lg:text-left">
        <a class="text-5xl font-bold" href="/">Amplify</a>
        <p class="py-6">
          Discover a revolutionary open market platform where you can buy and
          sell electricity in real-time.
        </p>
      </div>
      <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
        <form class="card-body">
          <h2 class="text-base font-semibold">Log in to your account</h2>

          <div class="form-control mt-1">
            <input
              id="email"
              type="email"
              placeholder="Email"
              class="input input-bordered"
              required
              bind:value={email}
            />
          </div>

          <div class="form-control mt-4">
            <input
              id="pw"
              type="password"
              placeholder="Password"
              class="input input-bordered"
              required
              bind:value={password}
            />
            <!-- button with icon -->
            <button
              type="button"
              on:click={showPassword}
              class="absolute inset-y-0 end-0 flex items-center z-20 px-3 cursor-pointer text-gray-400 rounded-e-md focus:outline-none focus:text-blue-600 dark:text-neutral-600 dark:focus:text-blue-500"
            >
              <svg
                class="shrink-0 size-3.5"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path
                  class="hs-password-active:hidden"
                  d="M9.88 9.88a3 3 0 1 0 4.24 4.24"
                ></path>
                <path
                  class="hs-password-active:hidden"
                  d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"
                ></path>
                <path
                  class="hs-password-active:hidden"
                  d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"
                ></path>
                <line
                  class="hs-password-active:hidden"
                  x1="2"
                  x2="22"
                  y1="2"
                  y2="22"
                ></line>
                <path
                  class="hidden hs-password-active:block"
                  d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"
                ></path>
                <circle
                  class="hidden hs-password-active:block"
                  cx="12"
                  cy="12"
                  r="3"
                ></circle>
              </svg>
            </button>
            {#if errormessage != ""}
              <p
                class="text-base font-semibold text-error rounded mt-12 fixed ml-1"
              >
                {errormessage}
              </p>
            {/if}
          </div>

          <div class="form-control mt-4">
            <button class="btn btn-primary" on:click={login}>Login</button>
          </div>

          <div class="form-control mt-3">
            <!-- <button class="btn btn-primary" on:click={signup}>Create an account</button> -->
            <a class="btn btn-outline btn-primary" href="/signup"
              >Create an account</a
            >
          </div>
        </form>
      </div>
    </div>
  </div>
</main>
