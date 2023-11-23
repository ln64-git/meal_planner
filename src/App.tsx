import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface Recipe {
  name: string;
  ingredients: string[];
  instructions: string[];
  note?: string;
}

function App() {
  const [ingredients, setIngredients] = useState("");
  const [response, setResponse] = useState<Recipe | string>("");

  const getRecipe = async () => {
    const ingredientsArray = ingredients.split(',').map((ingredient) => ingredient.trim());
    const recipeResponse = await invoke('get_recipe', {
      ingredients: ingredientsArray,
    });
    console.log(recipeResponse);
    setResponse(recipeResponse as Recipe);
  };

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
  
      {response && response instanceof Object && (
        <div>
          <h2>{response.name}</h2>
          <div>
            <h3>Ingredients:</h3>
            <ul>
              {response.ingredients.map((ingredient, index) => (
                <li key={index}>{ingredient}</li>
              ))}
            </ul>
          </div>
          <div>
            <h3>Instructions:</h3>
            <ol>
              {response.instructions.map((instruction, index) => (
                <li key={index}>{instruction}</li>
              ))}
            </ol>
          </div>
          {response.note && (
            <div>
              <h3>Note:</h3>
              <p>{response.note}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
          }
  
export default App;
