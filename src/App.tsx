import React from 'react';
import './App.css';

// TODO: subtitles on cards
export function TodoCard() {
  return (
    <li className='todo-card'>
      <div className='todo-card__label'>Long Silly Task</div>
      <button className='todo-card__check todo-card--groovy'>✔️</button>
      <button className='todo-card__delete todo-card--groovy'>🗑️</button>
    </li>
  );
}

export function TodoList() {
  // TODO1: implement local storage saving of tasks
  // TODO2: implement mongo db saving of tasks
  return (
    <ol className='todo-list'>
      <TodoCard />
      <TodoCard />
      <TodoCard />
      <AddTodoCard />
    </ol>
  );
}

export function AddTodoCard() {
  return (
    <li className='add-todo'>
      <input
        type='text'
        className='add-todo__small-input'
        placeholder='I gotta do...'
        name='taskname'
      ></input>
      <input type='submit' className='todo-card--groovy' value='➕'></input>
    </li>
  );
}

function App() {
  return <TodoList />;
}

export default App;
