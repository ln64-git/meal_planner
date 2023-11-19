<script lang="ts">
  import {onMount} from "svelte"
  import {invoke} from "@tauri-apps/api/tauri"

  let ingredients = ""
  let response = ""

  onMount(async () => {
    // This function will be called when the component is mounted
    async function getRecipe() {
      const ingredientsArray = ingredients
        .split(",")
        .map((ingredient) => ingredient.trim())
      console.log(ingredients)
      try {
        response = await invoke("get_recipe", {ingredients: ingredientsArray})
      } catch (error) {
        console.error("Error invoking get_recipe:", error)
        response = "Failed to get recipe"
      }
    }

    const recipeForm = document.getElementById(
      "recipe-form"
    ) as HTMLFormElement | null

    if (recipeForm) {
      recipeForm.addEventListener("submit", (event: Event) => {
        event.preventDefault()
        getRecipe()
      })
    } else {
      console.error("Recipe form element not found.")
    }
  })
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>Click on the Tauri, Vite, and Svelte logos to learn more.</p>

  <div class="row">
    <form id="recipe-form" class="row">
      <input
        id="greet-input"
        placeholder="Enter ingredients..."
        bind:value={ingredients}
      />
      <button type="submit">Get Recipe</button>
    </form>
    <p>{response}</p>
  </div>
</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
