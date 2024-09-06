<script>
  import "../../app.css";
  import { page } from "$app/stores";
  import { derived } from "svelte/store";
  import Cookies from "js-cookie";
  import { onMount } from "svelte";

  let loggedIn = false;

  let activebutton = "";

  const currentpath = derived(page, ($page) => $page.url.pathname);

  $: activebutton = $currentpath;

  function showModal() {
    if (activebutton == "/public/GridSimulation") {
      document.getElementById("my_modal_grid").showModal();
    } else if (activebutton == "/Main/Dashboard") {
      document.getElementById("my_modal_dash").showModal();
    } else if (activebutton == "/Main/BiddingMarket") {
      document.getElementById("help_modal").showModal();
    }
  }

  onMount(() => {
    const session = sessionStorage.getItem("Token");
    // console.log("Session id is: ", session);
    if (session) {
      loggedIn = true;
    } else {
      loggedIn = false;
      window.location.href = "/login";
    }
    // loggedIn = session === 'loggedIn';
  });

  async function removeAccount() {
    let data;
    try {
      const response = await fetch("http://localhost:8001/remove_account", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
      });
      data = await response.json();
      // console.log("Data received from remove account endpoint: ", data);
    } catch (error) {
      console.log("There was an error calling remove account:", error);
    }

    if (data.message == "Account successfully deleted") {
      Cookies.remove("session_id");
      window.location.href = "/login";
    }
  }

  function logout() {
    Cookies.remove("session_id");
    window.location.href = "/login";
  }

  let showMenu = false;
  let showIcon = true;

  function toggleHamburger() {
    showMenu = !showMenu;
    showIcon = !showIcon;
  }
</script>

<body class="w-full min-h-screen bg-gradient-to-b from-base-200 to-base-300">
  <header>
    <div class="navbar bg-base-100 border-b border-accent">
      <div class="navbar-start">
        <a class="btn btn-ghost md:text-xl font-normal xs:text-sm" href="/"
          >Amplify</a
        >
        <span class="md:text-xl xs:text-sm pl-4 font-normal">
          {activebutton == "/public/GridSimulation"
            ? "Simulation"
            : activebutton == "/Main/BiddingMarket"
              ? "Marketplace"
              : activebutton == "/Main/Dashboard"
                ? "Dashboard"
                : ""}
        </span>
      </div>

      <div class="navbar-center hidden lg:flex">
        <ul class="menu menu-horizontal px-1">
          <li class="px-2">
            <a
              class="w-28 justify-center btn-ghost"
              href="/public/GridSimulation">Grid</a
            >
          </li>
          <li class="px-2">
            <a class=" btn-ghost w-22" href="/Main/Dashboard">Dashboard</a>
          </li>
        </ul>
      </div>

      <div class="navbar-end">
        <div class="xs: hidden md:flex">
          <ul class="menu menu-horizontal px-3">
            <li class="px-2">
              <a class="w-22 btn-ghost" on:click={showModal}>Help</a>
            </li>
          </ul>
          <div class="dropdown dropdown-end">
            <div
              tabindex="0"
              role="button"
              class="btn btn-ghost rounded-btn font-normal"
            >
              Account
            </div>
            <ul
              class="menu dropdown-content bg-base-100 rounded-box z-[1] mt-4 w-52 p-2 shadow"
            >
              <button class="btn" onclick="removeaccount_modal.showModal()"
                >Remove Account</button
              >
              <button class="btn mt-2" on:click={logout}>Log out</button>
            </ul>
          </div>
        </div>

        <!-- mobile hamburger menu-->

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div on:click={toggleHamburger} class="md:hidden xs:flex">
          <!-- <button class="btn btn-square btn-ghost {showIcon ? '' : 'hidden'}">
        <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        class="inline-block h-5 w-5 stroke-current">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 6h16M4 12h16M4 18h16"></path>
      </svg>
      </button> -->

          <div class="navbar-end">
            <div class="dropdown">
              <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-5 w-5"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M4 6h16M4 12h16M4 18h7"
                  />
                </svg>
              </div>
              <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
              <ul
                tabindex="0"
                class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-28 p-2 shadow align-super fixed right-0"
              >
                <!-- svelte-ignore a11y-missing-attribute -->
                <li><a href="/">Landing</a></li>
                <!-- svelte-ignore a11y-missing-attribute -->
                <li><a href="/public/GridSimulation">Grid</a></li>
                <!-- svelte-ignore a11y-missing-attribute -->
                <li><a href="/Main/Dashboard">Dashboard</a></li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>

    <dialog id="my_modal_dash" class="modal">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Dashboard</h3>
        <p class="py-4">
          This is the central hub for controlling your nodes on the grid. <br />
          You can see your details, such as credit, on the left, and a list of your
          nodes and buy orders on the right. <br />
          If you plan on buying electricity, be sure to start by adding some funds
          to your account first. <br />
          Your credit is also where you will receive money for any electricity you
          sell, and you can withdraw from this at any time. <br />
          <br />
          Clicking on the "details" button on any of your nodes will open up more
          information about them, such as the amount of electricity it is allowed
          to consume/needs to produce. <br />
          Click on the "Transact with this node" button to go to the market page,
          where you can be part of our open market.
        </p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>

    <dialog id="my_modal_grid" class="modal">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Grid Simulation</h3>
        <p class="py-4">
          The grid simulation page contains an overview of the current state of
          the electrical grid. <br />
          On the map, you can see all the nodes that are connected to the simulated
          grid. <br />
          Clicking on one of these nodes will give you more information on them,
          and will show the voltage being generated at that point on the oscilloscope,
          on the right. <br />
          At the bottom you can see a few general statistics about the grid.
        </p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>

    <dialog id="help_modal" class="modal">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Marketplace</h3>
        <p class="py-4">
          The marketplace is designed to be much like any other trading website,
          where you can observe recent activity in the form of a price graph. <br
          />
          Here you can place buy orders, or sell your excess power to someone else
          connected to the grid.
        </p>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>

    <dialog id="removeaccount_modal" class="modal">
      <div class="modal-box">
        <h3 class="text-lg font-bold">Delete Account</h3>
        <p class="py-4">Are you sure you want to delete your account?</p>
        <div class="modal-action">
          <form method="dialog">
            <!-- if there is a button in form, it will close the modal -->
            <button class="btn bg-red-500" on:click={removeAccount}
              >Delete</button
            >
            <button class="btn bg-gray-600">Cancel</button>
          </form>
        </div>
      </div>

      <dialog id="my_modal_grid" class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg">Grid Simulation Page</h3>
          <p class="py-4">
            The grid simulation page contains an overview of the current state
            of the electrical grid. <br />
            On the map, you can see all the nodes that are connected to the simulated
            grid. <br />
            Clicking on one of these nodes will give you more information on them,
            and will show the voltage being generated at that point on the oscilloscope,
            on the right. <br />
            At the bottom you can see a few general statistics about the grid.
          </p>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

      <dialog id="my_modal_dash" class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg">Dashboard</h3>
          <p class="py-4">
            This is the central hub for controlling your nodes on the grid. <br
            />
            You can see your details, such as credit, on the left, and a list of
            your nodes and buy orders on the right. <br />
            If you plan on buying electricity, be sure to start by adding some funds
            to your account first. <br />
            Your credit is also where you will receive money for any electricity
            you sell, and you can withdraw from this at any time. <br />
            <br />
            Clicking on the "details" button on any of your nodes will open up more
            information about them, such as the amount of electricity it is allowed
            to consume/needs to produce. <br />
            Click on the "Transact with this node" button to go to the market page,
            where you can be part of our open market.
          </p>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

      <dialog id="help_modal" class="modal">
        <div class="modal-box">
          <h3 class="font-bold text-lg">Marketplace</h3>
          <p class="py-4">
            The marketplace is designed to be much like any other trading
            website, where you can observe recent activity in the form of a
            price graph. <br />
            Here you can place buy orders, or sell your excess power to someone else
            connected to the grid.
          </p>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

      <dialog id="removeaccount_modal" class="modal">
        <div class="modal-box">
          <h3 class="text-lg font-bold">Delete Account</h3>
          <p class="py-4">Are you sure you want to delete your account?</p>
          <div class="modal-action">
            <form method="dialog">
              <!-- if there is a button in form, it will close the modal -->
              <button class="btn bg-red-500" on:click={removeAccount}
                >Delete</button
              >
              <button class="btn bg-gray-600">Cancel</button>
            </form>
          </div>
        </div>
      </dialog>
    </dialog>
  </header>

  <main id="main" class="container mx-auto mt-8">
    {#if loggedIn}
      <slot />
    {/if}
  </main>
</body>
