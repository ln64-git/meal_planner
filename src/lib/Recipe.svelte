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

<div>
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
