import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import './index.css';
import { DashboardPage } from './pages/dashboard-page';

function App() {
  return <DashboardPage />;
}

createRoot(document.getElementById('root') as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
