
export interface Todo {
  id: number;
  name: string;
  completed: boolean;

}

const TODOS: string = 'TODOS';

export class TodoService {
    static async getAllTodo(): Promise<Todo[]> {
        return [];
    }

    static async saveTodo(todo: Todo): Promise<void> { }

    static async updateTodo(todo: Todo): Promise<void> { }

    static async deleteTodo(id: string): Promise<void> { }
}

// const useLocalStorage = (storageKey: string, fallbackState: any) => {
//   const [value, setValue] = React.useState(
//     JSON.parse(localStorage.getItem(storageKey) ?? '') ?? fallbackState
//   );
//
//   React.useEffect(() => {
//     localStorage.setItem(storageKey, JSON.stringify(value));
//   }, [value, storageKey]);
//
//   return [value, setValue];
// };

export class LocalStorageService {
    static async getAllTodo(): Promise<Todo[]> {
        const tasks: Todo[] = JSON.parse(localStorage.getItem(TODOS) ?? '') ?? [];
        return tasks;
    }

    static async saveTodo(todos: Todo[]): Promise<void> {
        localStorage.setItem(TODOS, JSON.stringify(todos));
    }

    static async updateTodo(todo: Todo): Promise<void> {
        const todos = await this.getAllTodo();
        const existing_todo: Todo = todos.find(t => t.id === todo.id) ?? {id: 0, name: "", completed: false};

    }

    static async deleteTodo(id: string): Promise<void> {

    }
}
