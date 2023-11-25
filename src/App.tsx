import React, {useState} from "react"
import {invoke} from "@tauri-apps/api/tauri"

interface Recipe {
  name: string
  ingredients: string[]
  instructions: string[]
  note?: string
}

function App() {
  const [ingredients, setIngredients] = useState("")
  const [response, setResponse] = useState<Recipe | string>("")

  const getRecipe = async () => {
    const ingredientsArray = ingredients
      .split(",")
      .map((ingredient) => ingredient.trim())
    const recipeResponse = await invoke("get_recipe", {
      ingredients: ingredientsArray,
    })
    console.log(recipeResponse)
    setResponse(recipeResponse as Recipe)
  }

  return (
    <div className='flex flex-col items-center justify-center h-screen bg-stone-800'>
      <form
        className='flex flex-row items-center w-full absolute top-0'
        onSubmit={async (e) => {
          e.preventDefault()
          await getRecipe()
        }}
      >
        <input
          onChange={(e) => setIngredients(e.currentTarget.value)}
          className='flex flex-grow'
          placeholder='Enter a name...'
        />
        <button type='submit'>Submit</button>
      </form>

      {response && response instanceof Object && (
        <div className='flex flex-col items-center bg-stone-900 text-white max-w-2xl mx-auto w-full overflow-auto p-6'>
          <div className='text-3xl text-center pb-4'>{response.name}</div>
          <div className='flex '>
            <div className='flex flex-col  pl-8'>
              <h3 className='font-bold text-sm'>Instructions:</h3>
              <ul>
                {response.ingredients.map((ingredient, index) => (
                  <div className='flex' key={index}>
                    <p className='pr-2'>{"\u2022"}</p>
                    <li>{ingredient}</li>
                  </div>
                ))}
              </ul>
            </div>
            <div className='flex flex-col  px-8'>
              <h3 className='font-bold text-center text-sm'>Instructions:</h3>
              <ol>
                {response.instructions.map((instruction, index) => (
                  <div className='flex break-words' key={index}>
                    <p className='pr-2'>{index + 1}.</p>
                    <li>{instruction}</li>
                  </div>
                ))}
              </ol>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default App
