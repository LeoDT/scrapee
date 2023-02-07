import {Suspense} from 'react';

import {Box, Flex} from '@chakra-ui/react';
import {Outlet} from 'react-router-dom';

import {MainTab} from './components/MainTab';

export function MainLayout(): JSX.Element {
  return (
    <Flex direction="column" justify="stretch" alignItems="stretch" h="100vh">
      <Box flexShrink={0} flexGrow={0}>
        <Suspense fallback="loading...">
          <MainTab />
        </Suspense>
      </Box>

      <Box flexGrow={1}>
        <Suspense fallback="loading...">
          <Outlet />
        </Suspense>
      </Box>
    </Flex>
  );
}
