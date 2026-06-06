<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import "./style.css";
    interface IUser {
        name: string;
        email: string;
        password: string;
        created_at: string;
        updated_at: string;
    }

    interface IUserRegister {
        name: string;
        email: string;
        password: string;
    }

    let user: IUser = $state({
        name: "",
        email: "",
        password: "",
        created_at: "",
        updated_at: "",
    });

    let userRegister: IUserRegister = $state({
        name: "",
        email: "",
        password: "",
    });
    let users: IUser[] = $state([]);
    async function register(user: IUserRegister): Promise<void> {
        try {
            const response = await invoke<void>("register", { user }).then(
                (res) => {
                    console.log(res);
                    return res;
                },
            );
            console.log(response);
            alert("Registration successful!");
        } catch (error) {
            console.error(error);
            alert("Registration failed!");
        }
    }

    async function getAll(): Promise<void> {
        console.log("getAll called");
        try {
            console.log("getAll called in try");
            users = await invoke<IUser[]>("get_all").then((res) => {
                console.log(res);
                return res;
            });
            // users = response as IUser[];
            console.log(users);
            alert("Get all successful!");
        } catch (error) {
            console.error(error);
            alert("Get all failed!");
        }
    }
</script>

<div class="register-container">
    <form
        onsubmit={(e) => {
            e.preventDefault();
            console.log(user);
            register(user);
        }}
        class="register-form"
    >
        <div class="form-group">
            <label for="username" class="form-label">Nome de usuário:</label>
            <input type="text" bind:value={user.name} class="form-control" />
        </div>
        <div class="form-group">
            <label for="email" class="form-label">Email:</label>
            <input type="email" bind:value={user.email} class="form-control" />
        </div>
        <div class="form-group">
            <label for="password" class="form-label">Senha:</label>
            <input
                type="password"
                bind:value={user.password}
                class="form-control"
            />
        </div>
        <div class="form-actions">
            <button class="btn btn-submit" type="submit">Cadastrar</button>
        </div>
    </form>

    <div>
        <button class="btn btn-info" type="button" onclick={getAll}
            >Get All</button
        >
    </div>
    <div class="users-container">
        {#each users as user}
            <div class="user">{user.name}</div>
            <div class="user">{user.email}</div>
            <div class="user">{user.created_at}</div>
            <div class="user">{user.updated_at}</div>
        {/each}
    </div>
</div>
