/* eslint-disable @typescript-eslint/no-empty-function */
import {useEffect, useState} from 'react';

import {register, unregister} from '@tauri-apps/api/globalShortcut';
import {Provider as JotaiProvider} from 'jotai/react';
import {createStore} from 'jotai/vanilla';
import {BrowserRouter, Routes, Route} from 'react-router-dom';

import {MainLayout} from './MainLayout';
import {Home} from './components/Home';
import {TabView} from './components/TabView';

function App() {
  const [store] = useState(() => {
    const store = createStore();

    return store;
  });

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

  useEffect(() => {
    if (import.meta.env.DEV) {
      unregister('Control+R')
        .then(() =>
          register('Control+R', () => {
            location.reload();
          }),
        )
        .then(
          () => {},
          () => {},
        );

      return () => {
        unregister('Control+R');
      };
    }
  }, []);

  return (
    <JotaiProvider store={store}>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<MainLayout />}>
            <Route index element={<Home />} />
            <Route path="/tab/:tabId" element={<TabView />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </JotaiProvider>
  );
}

export default App;
