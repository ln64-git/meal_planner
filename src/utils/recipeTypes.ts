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