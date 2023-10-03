import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import { TodoService } from './todos';

import { DATA_SOURCE } from './gottaconfig';

export let taskService: TodoService;

if (DATA_SOURCE === 'api') {
  // TaskService = require('./APIService').default;
} else if (DATA_SOURCE === 'local') {
  taskService = require('./todos').LocalStorageService;
} else {
  throw new Error('Invalid data source configuration.');
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
