<script lang="ts">
  import { goto } from "$app/navigation";
  import { User } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";

  let email: string = $state("");
  let error: string = $state("");

  async function submitLoginAttempt(event: Event) {
    event.preventDefault();

    const validEmail = await invoke("is_valid_email", { email: email });
    console.log(validEmail);
    if (!validEmail) {
      error = "Se requiere un correo ciencias unam para utilizar la aplicación";
      return;
    }

    window.localStorage.setItem("email", email);
    goto("/app");
  }
</script>

<div
  class="w-full max-w-4xl gap-3 container mx-auto flex flex-col items-center justify-center"
>
  <div>
    <a
      href="#"
      class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 card-hover divide-surface-200-800 block max-w-md divide-y overflow-hidden"
    >
      <header>
        <img src="/logo.png" class="aspect-[1/1] w-full" alt="banner" />
      </header>
    </a>
  </div>
  <div
    class="mx-auto card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block divide-y overflow-hidden"
  >
    <article class="space-y-4 p-4">
      <div>
        <h3 class="h3">Inicia sesión</h3>
      </div>
      <div>
        <form onsubmit={submitLoginAttempt}>
          <div class="input-group grid-cols-[auto_1fr_auto]">
            <div class="ig-cell preset-tonal">
              <User size={16} />
            </div>
            <input
              class="ig-input"
              bind:value={email}
              type="search"
              placeholder="Introduce tu correo..."
            />
            <button class="ig-btn btn preset-filled-primary-500" type="submit"
              >Iniciar</button
            >
          </div>
        </form>
      </div>
    </article>
    <footer class="flex items-center justify-between gap-4 p-4">
      <p>{error}</p>
    </footer>
  </div>
</div>
