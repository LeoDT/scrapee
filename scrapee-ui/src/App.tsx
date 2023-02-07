/* eslint-disable @typescript-eslint/no-empty-function */
import {useEffect} from 'react';

import {createStore, Provider as JotaiProvider} from 'jotai';
import {BrowserRouter, Routes, Route, Navigate} from 'react-router-dom';

import {MainLayout} from './MainLayout';
import {HOME_NAV} from './atoms/nav';
import {TabView} from './components/TabView';

const store = createStore();

function App() {
  // prevent links clicked with middle key or ctrl
  useEffect(() => {
    const listener = (e: MouseEvent) => {
      if (e.target instanceof HTMLAnchorElement) {
        if (e.button !== 0 || e.ctrlKey) {
          console.log(e);
          e.preventDefault();
        }
      }
    };

    document.addEventListener('click', listener);

    return () => document.removeEventListener('click', listener);
  }, []);

  return (
    <JotaiProvider store={store}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<MainLayout />}>
            <Route
              index
              element={<Navigate to={`/tab/${HOME_NAV.id}`} replace />}
            />
            <Route path="/tab/:navId" element={<TabView />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </JotaiProvider>
  );
}

export default App;
