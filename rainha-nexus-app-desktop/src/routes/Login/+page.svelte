<script lang="ts">
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";

    interface IUserSession {
        id: number;
        name: string;
        email: string;
    }

    let email = "";
    let password = "";

    async function login(): Promise<void> {
        try {
            const res = await invoke<IUserSession>("login", {
                email,
                password,
            });
            if (res) {
                alert("Login successful");
                goto("/Dashboard");
            }
        } catch (error) {
            console.error(error);
            alert("Login failed");
        }
    }
</script>

<div class="login-container">
    <h1>login</h1>
    <form class="login-form" onsubmit={login}>
        <div>
            <label for="email">Email</label>
            <input type="email" bind:value={email} id="email" />
        </div>
        <div>
            <label for="passwd">Password</label>
            <input type="password" bind:value={password} id="passwd" />
        </div>
        <button type="submit">Login</button>
    </form>
</div>
