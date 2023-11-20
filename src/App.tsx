import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [ingredients, setIngredients] = useState("");
  const [response, setResponse] = useState("");

  const getRecipe = async () => {
    console.log("Start");
    console.log(ingredients);
    const ingredientsArray = ingredients.split(',').map((ingredient) => ingredient.trim());
    
    try {
      const recipeResponse = await invoke('get_recipe', {
        ingredients: ingredientsArray,
      });
      setResponse(recipeResponse as string);
      console.log(response); // This might log the updated state
    } catch (error: any) {
      console.error('Error invoking get_recipe:', error);
      setResponse(error.toString());
    }
    console.log("Complete");
  };

  useEffect(() => {
    if (response !== '') {
      setResponse(response)
      console.log(response)
    }
  }, [response]);



  return (
    <div className="container">
      <form
        className="row"
        onSubmit={async (e) => {
          e.preventDefault();
          await getRecipe();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setIngredients(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Submit</button>
      </form>

      <p>{response}</p>
    </div>
  );
}

export default App;
