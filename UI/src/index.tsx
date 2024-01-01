import React from 'react';
import ReactDOM from 'react-dom/client';

import './index.scss';
import App from './Startup/App';
import reportWebVitals from './Startup/reportWebVitals';

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);
root.render(
    <React.StrictMode>
        <App/>
    </React.StrictMode>
);
reportWebVitals();
