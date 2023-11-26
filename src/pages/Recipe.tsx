import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { sampleRecipe } from '../assets/sampleRecipe.ts';

interface Recipe {
  name: string;
  ingredients: string[];
  instructions: string[];
  note?: string;
}

function App() {
  const [ingredients, setIngredients] = useState('');
  const [response, setResponse] = useState<Recipe | string>('');

  const getRecipe = async () => {
    const ingredientsArray = ingredients
      .split(',')
      .map((ingredient) => ingredient.trim());
    const recipeResponse = await invoke('get_recipe', {
      ingredients: ingredientsArray,
    });
    console.log(recipeResponse);
    setResponse(recipeResponse as Recipe);
  };

  return (
    <div className="flex h-screen flex-col items-center justify-center bg-stone-800">
      <form
        className="absolute top-0 flex w-full flex-row items-center"
        onSubmit={async (e) => {
          e.preventDefault();
          await getRecipe();
        }}
      >
        <input
          onChange={(e) => setIngredients(e.currentTarget.value)}
          className="flex flex-grow"
          placeholder="Enter a name..."
        />
        <button type="submit">Submit</button>
      </form>

      <div className="mx-auto flex w-full max-w-2xl flex-col items-center overflow-auto bg-stone-900 p-6 text-white">
        <div className="pb-4 text-center text-3xl">{sampleRecipe.name}</div>
        <div className="flex">
          <div className="flex flex-col pl-8 ">
            <h3 className="text-sm font-bold">Instructions:</h3>
            <ul>
              {sampleRecipe.ingredients.map((ingredient, index) => (
                <div className="flex" key={index}>
                  <p className="pr-2">{'\u2022'}</p>
                  <li>{ingredient}</li>
                </div>
              ))}
            </ul>
          </div>
          <div className="flex flex-col px-8">
            <h3 className="text-center text-sm font-bold">Instructions:</h3>
            <ol>
              {sampleRecipe.instructions.map((instruction, index) => (
                <div className="flex break-words" key={index}>
                  <p className="pr-2">{index + 1}.</p>
                  <li>{instruction}</li>
                </div>
              ))}
            </ol>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
