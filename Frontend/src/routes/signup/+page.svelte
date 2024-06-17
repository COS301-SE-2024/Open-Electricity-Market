<script>
    import logo from '$lib/assets/Logo.png';
    import {goto} from "$app/navigation";
    function create(){
        //add to database
        if(password == passwordValidate)
        {
            goto("/");
        }
    }
    function back(){
        goto("/");
    }
    let username = '';
    let password = '';
    let passwordValidate = '';
</script>
<body class = "h-screen w-max-xl bg-logSignup bg-cover bg-no-repeat bg-center">
    <div class = "card bg-black/90 h-full w-full flex flex-col space-y-12 items-center justify-center">
        <!--Ask for name and username too-->
        <img class= "h-auto max-w-full" alt = "Logo" src = {logo}/>
        <h1 class ="mb-4 text-heading1 font-heading1">Create an account with us!</h1>
        <div class = "flex flex-col space-y-8 items-center justify-center">
            <input type = "text" bind:value={username} placeholder="Username" class = "input bg-fillin text-fillinText font-fillin w-fillin h-fillin"/>
            <input type = "password" bind:value={password} placeholder="Password"class = "input bg-fillin text-fillinText font-fillin w-fillin h-fillin"/>
            <input type = "password" bind:value={passwordValidate} placeholder="Confirm Password"class = "input bg-fillin text-fillinText font-fillin w-fillin h-fillin"/>
        </div>
        <div class = "flex flex-col space-y-8 items-center justify-center">
            <button class = "btn bg-btn btn-outline text-btnTxt w-btn h-btn" on:click={create}>Create Account</button>
            <button class = "btn bg-btn btn-outline text-btnTxt w-btn h-btn" on:click={back}>Back</button>
        </div>
    </div>
</body>
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
    .body{
        color: white;
        display: flex;
        position: absolute;
        flex-flow: column;
        height: 100%;
        width: 100%;
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
    .fillin{
        height: 43px;
        width: 378px;
        margin: 18px;
        position: relative;
        top: 18px;
        border-radius: 6px;
        background-color: white;
        font-size:27px;
        color: black;
    }
    ::placeholder{
        color: black;
    }
    ::-ms-input-placeholder{
        color: black;
    }
    .buttons{
        background-color: #124E86;
        border: black;
        border-radius: 6px;
        color: white;
        margin: 19px;
        position: relative;
        top: 57px;
        text-align: center;
        font-size: 28px;
        height: 66px;
        width: 336px;
        transition-duration: 0.4s;
    }
    .buttons:hover{
        background-color: black;
        border: #124E86;
        border-radius: 6px;
        color: white;
        margin: 19px;
        position: relative;
        top: 57px;
        text-align: center;
        font-size: 28px;
        height: 66px;
        width: 336px;
    }
</style>