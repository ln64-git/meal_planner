import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface RecipeResponse {
  id: string;
  object: string;
  created: number;
  model: string;
  choices: Choice[];
}

interface Choice {
  index: number;
  message: Message;
}

interface Message {
  role: string;
  content: string;
}

interface Recipe {
  name: string;
  ingredients: string[];
  instructions: string[];
}

function App() {
  const [ingredients, setIngredients] = useState("");
  const [response, setResponse] = useState<RecipeResponse | string>("");

  const getRecipe = async () => {
    const ingredientsArray = ingredients.split(',').map((ingredient) => ingredient.trim());
      const recipeResponse = await invoke('get_recipe', {
        ingredients: ingredientsArray,
      });
      console.log(recipeResponse)
      setResponse(JSON.parse(recipeResponse) as RecipeResponse);
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

      {response && response.choices[0] && (
        <div>
          <h2>{response.choices[0]?.message.content}</h2>
          <div>
            <h3>Ingredients:</h3>
            <ul>
              {response.choices[0].message.content &&
                response.choices[0].message.content
                  .split("\n")
                  .filter((line) => line.startsWith("-"))
                  .map((ingredient, index) => (
                    <li key={index}>{ingredient.trim().slice(1)}</li>
                  ))}
            </ul>
          </div>
          <div>
            <h3>Instructions:</h3>
            <ol>
              {response.choices[0].message.content &&
                response.choices[0].message.content
                  .split("\n")
                  .filter((line) => line.match(/^\d+\./))
                  .map((instruction, index) => (
                    <li key={index}>{instruction.trim().slice(3)}</li>
                  ))}
            </ol>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
