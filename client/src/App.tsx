import React, { Component } from 'react';
import './App.css';
import { Todo } from './todos';

// TODO: subtitles on cards
export function TodoCard(task: Omit<Todo, 'id'> ) {
  const check: string = task.completed ? '✔️' : '☐';
  return (
    <li className='todo-card'>
      <div className='todo-card__label'>{task.name}</div>
      <button className='todo-card__check todo-card--groovy'>{check}</button>
      <button className='todo-card__delete todo-card--groovy'>🗑️</button>
    </li>
  );
}

interface TodoListState {
    todos: Todo[];
}

class TodoList extends Component<{}, TodoListState> {
  state = {
    todos: [{ id: 0, name: "Example", completed: false}],
  };

  addTodo = (newTodo: Todo) => {
      this.setState((prevState) => ({
        todos: [...prevState.todos, newTodo],
      }));
      
  };

  // TODO1: implement local storage saving of tasks
  // TODO2: implement mongo db saving of tasks
  render() {
      return (
        <ol className='todo-list'>
          {this.state.todos.map((todo, _) => (<TodoCard name={todo.name} completed={todo.completed} />))}
          <AddTodoCard addTodo={this.addTodo}/>
        </ol>
      );
  }
}

interface AddTodoProps {
    addTodo: (newTodo: Todo) => void;
}

class AddTodoCard extends Component<AddTodoProps> {
  state = {
      newTodoName: '',
  };
  handleKeyPress = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
      if (e.key === 'Enter') {
          e.preventDefault();
          this.createTodo(this.state.newTodoName);
      }
  };
  handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      this.setState({newTodoName: e.target.value});
  };
  handleClick = (_: React.MouseEvent<HTMLButtonElement>) => {
      this.createTodo(this.state.newTodoName);
  };
  createTodo = (name: string) => {
      let newTodo: Todo = { id: 0, name: name, completed: false };
      this.props.addTodo(newTodo);
      this.setState({newTodoName: ''});
  };

  render() {
    return (
      <li className='add-todo'>
        <textarea
          className='add-todo__small-input'
          onKeyDown={this.handleKeyPress}
          onChange={this.handleChange}
          placeholder='I gotta do...'
          value={this.state.newTodoName}
        />
        <button className='todo-card--groovy' onClick={this.handleClick} >➕</button>
      </li>
    );
  }
}

function App() {
  return <TodoList />;
}

export default App;
