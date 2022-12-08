import {HStack, Box, Link} from '@chakra-ui/react';
import {useAtomValue} from 'jotai/react';
import {Link as RouterLink} from 'react-router-dom';

import {navsAtom} from '../atoms/nav';

export function MainTab(): JSX.Element {
  const navs = useAtomValue(navsAtom);

  return (
    <HStack bgColor="gray.800">
      {navs.map(nav => (
        <Box key={nav.id}>
          <Link as={RouterLink} to={nav.link ?? `/${nav.id}`} color="white">
            {nav.id}
          </Link>
        </Box>
      ))}
    </HStack>
  );
}
