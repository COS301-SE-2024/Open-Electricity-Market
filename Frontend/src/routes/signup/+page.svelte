<script>
  import logo from "$lib/assets/Logo.png";
  import { goto } from "$app/navigation";
  import Cookies from "js-cookie";
  import {
    API_URL_GRID,
    API_URL_MARKET,
    COOKIE_DOMAIN,
    API_URL_AGENT,
  } from "$lib/config.js";

  let email = "";
  let firstname = "";
  let surname = "";
  let password = "";
  let password2 = "";
  let errormessage = "";
  let validEmail = false;
  let validPassword = false;
  let data = {};

  // RFC 2822 standard email validation pattern
  var emailRegex =
    /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;

  // 8 characters, at least one lowercase, one uppercase, one symbol and one numeric character
  // might separate it out to make it reactive and indicate which requirements are not met to the user
  var passwordRegex = /^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[a-zA-Z]).{8,}$/;

  async function validateEmail() {
    const emailElement = document.getElementById("email");
    if (!email.match(emailRegex)) {
      validEmail = false;
      errormessage = "Please enter a valid email address.";

      emailElement.classList.add("input-error");

      return;
    }
    // maybe have an email specific error message
    errormessage = "";
    emailElement.classList.remove("input-error");

    validEmail = true;
  }

  function passwordCreationHelp() {
    var pwElement = document.getElementById("pw");
    if (!password.match(passwordRegex)) {
      errormessage =
        "Password requires at least 8 characters, uppercase and lowercase, a symbol and a number";
      pwElement.classList.add("input-error");
      validPassword = false;
    } else {
      errormessage = "";
      pwElement.classList.remove("input-error");
      validPassword = true;
    }
  }

  function showPassword() {
    var input = document.getElementById("pw");
    if (input.type === "password") {
      input.type = "text";
    } else {
      input.type = "password";
    }
  }

  function showPassword2() {
    var input = document.getElementById("pw2");
    if (input.type === "password") {
      input.type = "text";
    } else {
      input.type = "password";
    }
  }

  async function create() {
    if (!validEmail || !validPassword) {
      errormessage = "Invalid email or password";
      return;
    }

    if (password == password2) {
      const res = await fetch(`${API_URL_MARKET}/register`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
        },
        body: JSON.stringify({
          email: email,
          first_name: firstname,
          last_name: surname,
          password: password,
        }),
      });
      const json = await res.json();
      if (json.status == "ok") {
        sessionStorage.setItem("Token", json.data.token);
        await addAgent();
        goto("/Main/Dashboard");
      } else {
        errormessage = "An error occured";
      }
    } else {
      errormessage = "Passwords must match";
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
        },
        body: JSON.stringify(details),
        credentials: "include",
      });
      const fdata = await res.json();
      console.log("Add agent endpoint: ", fdata);
    } catch (error) {
      console.log(
        "There was an error with the add_agent endpoint when creating account: ",
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
      class="hero-content max-w-screen-md max-h-min flex-col lg:flex-row-reverse rounded-2xl bg-base-100"
    >
      <div class="text-center lg:text-left">
        <h1 class="text-5xl font-bold">Amplify</h1>
        <p class="py-6">
          Discover a revolutionary open market platform where you can buy and
          sell electricity in real-time.
        </p>
      </div>
      <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
        <form class="card-body">
          <h2 class="text-base font-semibold ml-1">Create an account</h2>

          <div class="form-control">
            <input
              id="email"
              type="email"
              placeholder="Email"
              class="input input-bordered"
              required
              bind:value={email}
              on:change={validateEmail}
            />
          </div>

          <div class="form-control mt-4">
            <input
              type="text"
              placeholder="First name"
              class="input input-bordered"
              required
              bind:value={firstname}
            />
          </div>

          <div class="form-control mt-4">
            <input
              type="text"
              placeholder="Surname"
              class="input input-bordered"
              required
              bind:value={surname}
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
              on:input={passwordCreationHelp}
            />

            <!-- button with icon (this doesnt work) -->
            <!-- <button type="button" on:click={showPassword} class="absolute inset-y-0 end-0 flex items-center z-20 px-3 cursor-pointer text-gray-400 rounded-e-md focus:outline-none focus:text-blue-600 dark:text-neutral-600 dark:focus:text-blue-500">
                  <svg class="shrink-0 size-3.5" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path class="hs-password-active:hidden" d="M9.88 9.88a3 3 0 1 0 4.24 4.24"></path>
                    <path class="hs-password-active:hidden" d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"></path>
                    <path class="hs-password-active:hidden" d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"></path>
                    <line class="hs-password-active:hidden" x1="2" x2="22" y1="2" y2="22"></line>
                    <path class="hidden hs-password-active:block" d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"></path>
                    <circle class="hidden hs-password-active:block" cx="12" cy="12" r="3"></circle>
                  </svg>
                </button> -->
          </div>

          <!-- Checkbox (these work)-->
          <div class="flex ml-2">
            <input
              on:click={showPassword}
              id="hs-toggle-password-checkbox"
              type="checkbox"
              class="shrink-0 mt-0.5 border-gray-200 rounded text-blue-600 focus:ring-blue-500 dark:bg-neutral-800 dark:border-neutral-700 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800"
            />
            <label
              for="hs-toggle-password-checkbox"
              class="text-sm text-gray-500 ms-3 dark:text-neutral-400"
              >Show</label
            >
          </div>

          <div class="form-control mt-2">
            <input
              id="pw2"
              type="password"
              placeholder="Re-enter password"
              class="input input-bordered"
              required
              bind:value={password2}
            />
          </div>

          <!-- Checkbox -->
          <div class="flex ml-2">
            <input
              on:click={showPassword2}
              id="hs-toggle-password-checkbox-2"
              type="checkbox"
              class="shrink-0 mt-0.5 border-gray-200 rounded text-blue-600 focus:ring-blue-500 dark:bg-neutral-800 dark:border-neutral-700 dark:checked:bg-blue-500 dark:checked:border-blue-500 dark:focus:ring-offset-gray-800"
            />
            <label
              for="hs-toggle-password-checkbox-2"
              class="text-sm text-gray-500 ms-3 dark:text-neutral-400"
              >Show</label
            >
          </div>

          {#if errormessage != ""}
            <p class="text-base font-semibold text-error rounded mt-2">
              {errormessage}
            </p>
          {/if}

          <div class="form-control mt-4">
            <button class="btn btn-primary" on:click={create}
              >Create account</button
            >
          </div>

          <div class="form-control mt-3">
            <!-- <button class="btn btn-primary" on:click={back}>I already have an account</button> -->
            <a class="btn btn-outline btn-primary" href="/login"
              >I already have an account</a
            >
          </div>
        </form>
      </div>
    </div>
  </div>
</main>
