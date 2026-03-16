import { useEffect } from 'react';
import { useAuth } from './auth/AuthContext';
import LoginPage from './auth/LoginPage';
import Layout from './components/Layout';

function App() {
  const { user, loading } = useAuth();

  useEffect(() => {
    if (!loading && !user) {
      window.history.replaceState(null, '', '/login');
    }
  }, [user, loading]);

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
