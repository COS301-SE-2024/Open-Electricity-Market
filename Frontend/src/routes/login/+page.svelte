<script>
    import logo from '$lib/assets/Logo.png';
    //import pylonBack from '$lib/assets/pylonBack.png';
    import Button from "/mnt/c/University/Year3/Semester1/COS301/Capstone/CapstoneWork/Open-Electricity-Market/Frontend/src/routes/Button.svelte";
    //let pylonBack = '$lib/assets/pylonBack.png';
    let username = '';
    let password = '';
</script>

<div class = "background">
    <div class = "body">
        <img class= "logo" alt = "Logo" src = {logo}/>
        <h1>Welcome To OpenElectricityMarket!</h1>
        <h2>Login into account:</h2>
        <input class = "fillin" type = "text" color = "black" bind:value={username} placeholder="Username here..."/>
        <input class = "fillin" type = "text" size = "35" bind:value={password} placeholder="Password here..."/>
        <Button class = "login">Login</Button>
        <Button class = "signup">Signup</Button>
    </div>
</div>

<style>
    @font-face{
        font-family: 'NATS';
        font-style: normal;
        font-weight: 50;
        src: url("/fonts/NATS-Regular.ttf");
    }
    .background{
        position: absolute;
        top: 0;
        left: 0;
        height: 100%;
        width: 100%;
        background-image: url('$lib/assets/pylonBack.png');
        background-size: cover;
    }
    
    .body
    {
        color: white;
        display: flex;
        position: absolute;
        flex-flow: column;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.93);
        align-items: center;
    }
    .logo{
        position: relative;
        top: 20px;
    }
    h1{
        font-weight: normal;
        font-size: 50px;
        top: 24px;
        position: relative;
        font-family: NATS;
        letter-spacing: 9px;
    }
    h2{
        font-weight: normal;
        font-size: 33px;
        top: -28px;
        position: relative;
        font-family: NATS;
        letter-spacing: 10px;
    }
    .fillin{
        height: 43px;
        width: 378px;
        margin: 18px;
        border-radius: 6px;
        background-color: white;
        font-size:47px;
        color: black;
    }
    
</style>