import { useEffect } from 'react';
import { useAuth } from './auth/AuthContext';
import { ApiError } from './api/client';
import LoginPage from './auth/LoginPage';
import Layout from './components/Layout';

function App() {
  const { user, loading, signOut } = useAuth();

  useEffect(() => {
    if (!loading && !user) {
      window.history.replaceState(null, '', '/login');
    }
  }, [user, loading]);

  useEffect(() => {
    function handleUnhandledRejection(event: PromiseRejectionEvent) {
      if (event.reason instanceof ApiError && event.reason.status === 401) {
        event.preventDefault();
        signOut();
      }
    }
    window.addEventListener('unhandledrejection', handleUnhandledRejection);
    return () => window.removeEventListener('unhandledrejection', handleUnhandledRejection);
  }, [signOut]);

  if (loading) {
    return (
      <div style={{
        minHeight: '100vh',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: '#1a1a1a',
        color: '#e0e0e0',
        fontFamily: 'monospace'
      }}>
        Loading...
      </div>
    );
  }

  if (!user) {
    return <LoginPage />;
  }

  return <Layout />;
}

export default App;
