import {Outlet} from 'react-router-dom';

import {MainTab} from './components/MainTab';

export function MainLayout(): JSX.Element {
  return (
    <div>
      <MainTab />

      <Outlet />
    </div>
  );
}
