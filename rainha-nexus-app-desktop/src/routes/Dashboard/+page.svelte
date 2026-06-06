<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let user: any;

    onMount(async () => {
        console.log("ok");
        // user = await invoke("current_user").then((res) => {
        //     console.log("em dagsboard o res");
        //     console.log(res);
        //     return res;
        // });
    });
</script>

<div class="dashboard">
    <nav>
        <a href="/Login">Login</a>
    </nav>
    <h1>Dashboard</h1>
    <h2>{user.name}</h2>
</div>
